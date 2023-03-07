use clap;

/// JJumping is the first step towards reaching new heights.
#[derive(clap::Parser, Debug)]
#[command(
    author, 
    version, 
    about, 
    long_about = None, 
    args_conflicts_with_subcommands = true,
    arg_required_else_help(true),
)]
pub struct Args {
    #[clap(subcommand)]
    sub: Option<Commands>,

    #[clap(flatten)]
    goto: Option<CommandGoto>,
}

#[derive(clap::Parser, Debug)]
pub enum Commands {
    /// Create a new portal.
    Add(CommandAdd),
    /// Use one of your portals.
    Goto(CommandGoto),
}

#[derive(clap::Parser, Debug)]
pub struct CommandAdd {
    /// Where do you want your portal to point at?
    destination: String,

    /// Assign one or multiple names to your portal.
    names: Vec<String>,
}

#[derive(clap::Parser, Debug)]
pub struct CommandGoto {
    /// Use a portal to jump to your target directory and conquer the (un)known.
    name: String,
}
