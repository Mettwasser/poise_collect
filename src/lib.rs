pub use inventory;
pub use poise_collect_macro::command;

#[macro_export]
macro_rules! init {
    ($t:ty, $error:ty) => {
        pub type Command = poise::Command<$t, $error>;

        pub struct RegisteredCommand {
            pub constructor: fn() -> Command,
        }

        $crate::inventory::collect!(RegisteredCommand);

        pub fn all_commands() -> Vec<Command> {
            $crate::inventory::iter::<RegisteredCommand>
                .into_iter()
                .map(|registered| (registered.constructor)())
                .collect()
        }
    };
}
