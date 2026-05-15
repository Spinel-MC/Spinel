use crate::data_type::DataType;
use std::io::{self, Read, Write};
use std::time::{Duration, Instant, SystemTime};

#[derive(Debug, Clone, Copy)]
pub struct NetworkInstant(pub Instant);

impl DataType for NetworkInstant {
    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()> {
        let now_instant = Instant::now();
        let now_system = SystemTime::now();

        let system_time = if self.0 >= now_instant {
            now_system + self.0.duration_since(now_instant)
        } else {
            now_system - now_instant.duration_since(self.0)
        };

        let millis = system_time
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
            .as_millis() as i64;

        millis.encode(w)
    }

    fn decode<R: Read>(r: &mut R) -> io::Result<Self> {
        let millis = i64::decode(r)?;
        let target_system = SystemTime::UNIX_EPOCH
            .checked_add(Duration::from_millis(millis as u64))
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Instant out of range"))?;

        let now_instant = Instant::now();
        let now_system = SystemTime::now();

        let target_instant = match target_system.duration_since(now_system) {
            Ok(delta) => now_instant + delta,
            Err(err) => now_instant - err.duration(),
        };

        Ok(Self(target_instant))
    }
}
