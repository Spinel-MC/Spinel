use crate::world::{
    BlockFaceDirection, BlockPosition, BlockShapeBox, BlockState, Chunk, ChunkPosition,
};
use std::collections::{HashMap, VecDeque};

const SECTION_SIZE: i32 = 16;

pub(super) struct WorldLighting;

impl WorldLighting {
    pub(super) fn relight(
        chunks: &mut HashMap<ChunkPosition, Chunk>,
        has_skylight: bool,
        requested_positions: Option<&[ChunkPosition]>,
    ) -> Vec<ChunkPosition> {
        let affected_positions = Self::affected_positions(chunks, requested_positions);
        let occlusion_maps = if has_skylight {
            chunks
                .iter_mut()
                .filter(|(_, chunk)| chunk.is_loaded() && chunk.is_lighting_chunk())
                .map(|(position, chunk)| (*position, *chunk.sky_occlusion_map()))
                .collect()
        } else {
            HashMap::new()
        };
        let block_light = Self::block_light(chunks);
        let sky_light = Self::sky_light(chunks, &occlusion_maps);

        chunks
            .values_mut()
            .filter(|chunk| chunk.is_loaded() && chunk.is_lighting_chunk())
            .for_each(Chunk::clear_light);
        block_light.into_iter().for_each(|(position, level)| {
            if let Some(chunk) = chunks
                .get_mut(&ChunkPosition::from(position))
                .filter(|chunk| chunk.is_lighting_chunk())
            {
                chunk.set_block_light(position, level);
            }
        });
        sky_light.into_iter().for_each(|(position, level)| {
            if let Some(chunk) = chunks
                .get_mut(&ChunkPosition::from(position))
                .filter(|chunk| chunk.is_lighting_chunk())
            {
                chunk.set_sky_light(position, level);
            }
        });
        chunks
            .values_mut()
            .filter(|chunk| chunk.is_loaded() && chunk.is_lighting_chunk())
            .for_each(Chunk::validate_light);
        affected_positions
    }

    fn block_light(chunks: &HashMap<ChunkPosition, Chunk>) -> HashMap<BlockPosition, u8> {
        let mut light = HashMap::new();
        let mut propagation = VecDeque::new();

        chunks
            .values()
            .filter(|chunk| chunk.is_loaded() && chunk.is_lighting_chunk())
            .for_each(|chunk| {
                Self::chunk_positions(chunk).for_each(|position| {
                    let level = chunk.block_state(position).light_emission();
                    if level == 0 {
                        return;
                    }
                    light.insert(position, level);
                    propagation.push_back((position, level));
                });
            });

        Self::propagate(chunks, light, propagation)
    }

    fn affected_positions(
        chunks: &HashMap<ChunkPosition, Chunk>,
        requested_positions: Option<&[ChunkPosition]>,
    ) -> Vec<ChunkPosition> {
        let Some(requested_positions) = requested_positions else {
            return chunks
                .iter()
                .filter(|(_, chunk)| chunk.is_loaded() && chunk.is_lighting_chunk())
                .map(|(position, _)| *position)
                .collect();
        };
        requested_positions
            .iter()
            .flat_map(|position| {
                (-1..=1).flat_map(move |offset_x| {
                    (-1..=1).map(move |offset_z| {
                        ChunkPosition::new(position.x + offset_x, position.z + offset_z)
                    })
                })
            })
            .filter(|position| {
                chunks
                    .get(position)
                    .is_some_and(|chunk| chunk.is_loaded() && chunk.is_lighting_chunk())
            })
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect()
    }

    fn sky_light(
        chunks: &HashMap<ChunkPosition, Chunk>,
        occlusion_maps: &HashMap<ChunkPosition, [i32; 256]>,
    ) -> HashMap<BlockPosition, u8> {
        let mut light = HashMap::new();
        let mut propagation = VecDeque::new();

        chunks
            .values()
            .filter(|chunk| chunk.is_loaded() && chunk.is_lighting_chunk())
            .for_each(|chunk| {
                let maximum_y = chunk.max_section() * SECTION_SIZE - 1;
                let Some(occlusion_map) =
                    occlusion_maps.get(&ChunkPosition::new(chunk.x(), chunk.z()))
                else {
                    return;
                };
                (0..SECTION_SIZE).for_each(|local_x| {
                    (0..SECTION_SIZE).for_each(|local_z| {
                        let global_x = chunk.x() * SECTION_SIZE + local_x;
                        let global_z = chunk.z() * SECTION_SIZE + local_z;
                        let occlusion_height =
                            occlusion_map[(local_z * SECTION_SIZE + local_x) as usize];
                        for y in (occlusion_height..=maximum_y).rev() {
                            let position = BlockPosition::new(global_x, y, global_z);
                            light.insert(position, 15);
                            propagation.push_back((position, 15));
                        }
                    });
                });
            });

        Self::propagate(chunks, light, propagation)
    }

    fn propagate(
        chunks: &HashMap<ChunkPosition, Chunk>,
        mut light: HashMap<BlockPosition, u8>,
        mut propagation: VecDeque<(BlockPosition, u8)>,
    ) -> HashMap<BlockPosition, u8> {
        while let Some((position, level)) = propagation.pop_front() {
            if level <= 1 {
                continue;
            }
            let Some(current_state) = Self::state(chunks, position) else {
                continue;
            };
            Direction::ALL.into_iter().for_each(|direction| {
                let target_position = direction.offset(position);
                let Some(target_state) = Self::state(chunks, target_position) else {
                    return;
                };
                let next_level = level - 1;
                if light
                    .get(&target_position)
                    .is_some_and(|existing_level| *existing_level >= next_level)
                {
                    return;
                }
                if Self::blocks_light(current_state, target_state, direction) {
                    return;
                }
                light.insert(target_position, next_level);
                propagation.push_back((target_position, next_level));
            });
        }
        light
    }

    fn state(
        chunks: &HashMap<ChunkPosition, Chunk>,
        position: BlockPosition,
    ) -> Option<BlockState> {
        chunks
            .get(&ChunkPosition::from(position))
            .filter(|chunk| chunk.is_loaded() && chunk.is_lighting_chunk())
            .map(|chunk| chunk.block_state(position))
    }

    fn blocks_light(current: BlockState, target: BlockState, direction: Direction) -> bool {
        if current.block().is_air() && target.block().is_air() {
            return false;
        }
        let current_face = direction.face();
        let target_face = direction.opposite_face();
        if current.light_emission() > 0 {
            return Self::face_is_covered(target.face_occlusion_shape(target_face), target_face);
        }
        Self::faces_cover(
            current.face_occlusion_shape(current_face),
            current_face,
            target.face_occlusion_shape(target_face),
            target_face,
        )
    }

    fn faces_cover(
        current_boxes: &[BlockShapeBox],
        current_face: BlockFaceDirection,
        target_boxes: &[BlockShapeBox],
        target_face: BlockFaceDirection,
    ) -> bool {
        let mut rectangles = current_boxes
            .iter()
            .map(|shape_box| FaceRectangle::from_box(shape_box, current_face))
            .collect::<Vec<_>>();
        rectangles.extend(
            target_boxes
                .iter()
                .map(|shape_box| FaceRectangle::from_box(shape_box, target_face)),
        );
        Self::rectangles_cover_face(&rectangles)
    }

    fn face_is_covered(shape_boxes: &[BlockShapeBox], face: BlockFaceDirection) -> bool {
        let rectangles = shape_boxes
            .iter()
            .map(|shape_box| FaceRectangle::from_box(shape_box, face))
            .collect::<Vec<_>>();
        Self::rectangles_cover_face(&rectangles)
    }

    fn rectangles_cover_face(rectangles: &[FaceRectangle]) -> bool {
        if rectangles.is_empty() {
            return false;
        }
        let mut first_axis = vec![0.0, 1.0];
        let mut second_axis = vec![0.0, 1.0];
        rectangles.iter().for_each(|rectangle| {
            first_axis.extend([rectangle.minimum_first, rectangle.maximum_first]);
            second_axis.extend([rectangle.minimum_second, rectangle.maximum_second]);
        });
        first_axis.sort_by(f64::total_cmp);
        second_axis.sort_by(f64::total_cmp);
        first_axis.dedup();
        second_axis.dedup();

        first_axis.windows(2).all(|first_interval| {
            second_axis.windows(2).all(|second_interval| {
                let first = (first_interval[0] + first_interval[1]) / 2.0;
                let second = (second_interval[0] + second_interval[1]) / 2.0;
                rectangles
                    .iter()
                    .any(|rectangle| rectangle.contains(first, second))
            })
        })
    }

    fn chunk_positions(chunk: &Chunk) -> impl Iterator<Item = BlockPosition> + '_ {
        let minimum_y = chunk.min_section() * SECTION_SIZE;
        let maximum_y = chunk.max_section() * SECTION_SIZE;
        (minimum_y..maximum_y).flat_map(move |y| {
            (0..SECTION_SIZE).flat_map(move |local_z| {
                (0..SECTION_SIZE).map(move |local_x| {
                    BlockPosition::new(
                        chunk.x() * SECTION_SIZE + local_x,
                        y,
                        chunk.z() * SECTION_SIZE + local_z,
                    )
                })
            })
        })
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Down,
    Up,
    North,
    South,
    West,
    East,
}

impl Direction {
    const ALL: [Self; 6] = [
        Self::Down,
        Self::Up,
        Self::North,
        Self::South,
        Self::West,
        Self::East,
    ];

    fn offset(self, position: BlockPosition) -> BlockPosition {
        let (x, y, z) = match self {
            Self::Down => (position.x, position.y - 1, position.z),
            Self::Up => (position.x, position.y + 1, position.z),
            Self::North => (position.x, position.y, position.z - 1),
            Self::South => (position.x, position.y, position.z + 1),
            Self::West => (position.x - 1, position.y, position.z),
            Self::East => (position.x + 1, position.y, position.z),
        };
        BlockPosition::new(x, y, z)
    }

    const fn face(self) -> BlockFaceDirection {
        match self {
            Self::Down => BlockFaceDirection::Down,
            Self::Up => BlockFaceDirection::Up,
            Self::North => BlockFaceDirection::North,
            Self::South => BlockFaceDirection::South,
            Self::West => BlockFaceDirection::West,
            Self::East => BlockFaceDirection::East,
        }
    }

    const fn opposite_face(self) -> BlockFaceDirection {
        match self {
            Self::Down => BlockFaceDirection::Up,
            Self::Up => BlockFaceDirection::Down,
            Self::North => BlockFaceDirection::South,
            Self::South => BlockFaceDirection::North,
            Self::West => BlockFaceDirection::East,
            Self::East => BlockFaceDirection::West,
        }
    }
}

struct FaceRectangle {
    minimum_first: f64,
    maximum_first: f64,
    minimum_second: f64,
    maximum_second: f64,
}

impl FaceRectangle {
    fn from_box(shape_box: &BlockShapeBox, face: BlockFaceDirection) -> Self {
        let (minimum_first, maximum_first, minimum_second, maximum_second) = match face {
            BlockFaceDirection::Down | BlockFaceDirection::Up => (
                shape_box.min_x,
                shape_box.max_x,
                shape_box.min_z,
                shape_box.max_z,
            ),
            BlockFaceDirection::North | BlockFaceDirection::South => (
                shape_box.min_x,
                shape_box.max_x,
                shape_box.min_y,
                shape_box.max_y,
            ),
            BlockFaceDirection::West | BlockFaceDirection::East => (
                shape_box.min_z,
                shape_box.max_z,
                shape_box.min_y,
                shape_box.max_y,
            ),
        };
        Self {
            minimum_first,
            maximum_first,
            minimum_second,
            maximum_second,
        }
    }

    fn contains(&self, first: f64, second: f64) -> bool {
        first >= self.minimum_first
            && first <= self.maximum_first
            && second >= self.minimum_second
            && second <= self.maximum_second
    }
}
