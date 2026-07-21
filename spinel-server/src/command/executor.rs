use crate::command::{CommandContext, CommandExecutionResult, CommandSender};
use crate::server::MinecraftServer;

pub type CommandExecutorFunction = for<'a> fn(
    &mut MinecraftServer,
    CommandSender<'a>,
    &mut CommandContext,
) -> CommandExecutionResult;

pub type CommandExecutorMethod<T> = for<'a> fn(
    &T,
    &mut MinecraftServer,
    CommandSender<'a>,
    &mut CommandContext,
) -> CommandExecutionResult;

type ErasedCommandExecutor = unsafe fn(
    usize,
    usize,
    &mut MinecraftServer,
    CommandSender<'_>,
    &mut CommandContext,
) -> CommandExecutionResult;

#[derive(Clone, Copy)]
pub struct CommandExecutor {
    receiver: usize,
    executor: usize,
    dispatch: ErasedCommandExecutor,
}

impl CommandExecutor {
    pub fn from_function(executor: CommandExecutorFunction) -> Self {
        Self {
            receiver: 0,
            executor: executor as usize,
            dispatch: dispatch_command_function,
        }
    }

    pub fn from_method<T>(receiver: &T, executor: CommandExecutorMethod<T>) -> Self {
        Self {
            receiver: receiver as *const T as usize,
            executor: executor as usize,
            dispatch: dispatch_command_method::<T>,
        }
    }

    pub fn execute(
        self,
        server: &mut MinecraftServer,
        sender: CommandSender<'_>,
        context: &mut CommandContext,
    ) -> CommandExecutionResult {
        unsafe { (self.dispatch)(self.receiver, self.executor, server, sender, context) }
    }
}

unsafe fn dispatch_command_function(
    _receiver: usize,
    executor: usize,
    server: &mut MinecraftServer,
    sender: CommandSender<'_>,
    context: &mut CommandContext,
) -> CommandExecutionResult {
    let executor: CommandExecutorFunction = unsafe { std::mem::transmute(executor) };
    executor(server, sender, context)
}

unsafe fn dispatch_command_method<T>(
    receiver: usize,
    executor: usize,
    server: &mut MinecraftServer,
    sender: CommandSender<'_>,
    context: &mut CommandContext,
) -> CommandExecutionResult {
    let receiver = unsafe { &*(receiver as *const T) };
    let executor: CommandExecutorMethod<T> = unsafe { std::mem::transmute(executor) };
    executor(receiver, server, sender, context)
}
