
cmd_use!();

pub struct ConfCmd {

}

impl Cmd  for ConfCmd
{

    fn execute(&self,res: &ResBox, context : &mut Context)->BoolR
    {
        res.conf(context) 
    }
}
impl CmdLoader<ConfCmd> for ConfCmd
{
    fn load() -> ConfCmd { ConfCmd{ } }
    fn key() -> String { String::from("conf") }
}
impl CmdDesp for ConfCmd {
    fn cmd_info(&self) -> String { format!("Cmd,ConfCmd") }
    fn cmd_name(&self) -> String { ConfCmd::key() }
}

pub struct StartCmd {

}

impl Cmd for StartCmd 
{
    fn execute(&self,res: &ResBox, context : &mut Context)->BoolR
    {
        res.start(context) 
    }

}
impl CmdLoader<StartCmd> for StartCmd
{
    fn load() -> StartCmd { StartCmd{ } }
    fn key() -> String { String::from("start") }
}
impl CmdDesp for StartCmd {
    fn cmd_info(&self) -> String { format!("Cmd,StartCmd") }
    fn cmd_name(&self) -> String { StartCmd::key() }
}


pub struct StopCmd {

}
impl CmdLoader<StopCmd> for StopCmd
{
    fn load() -> StopCmd { StopCmd{ } }
    fn key() -> String { String::from("stop") }
}
impl CmdDesp for StopCmd {
    fn cmd_info(&self) -> String { format!("Cmd,StopCmd") }
    fn cmd_name(&self) -> String { StopCmd::key() }
}

impl Cmd for StopCmd 
{
    fn execute(&self,res: &ResBox, context : &mut Context)->BoolR
    {
        res.stop(context) 
    }

}


pub struct CleanCmd {

}

impl Cmd for CleanCmd 
{
    fn execute(&self,res: &ResBox, context : &mut Context)->BoolR
    {
        res.clean(context) 
    }

}
impl CmdLoader<CleanCmd> for CleanCmd
{
    fn load() -> CleanCmd { CleanCmd{ } }
    fn key() -> String { String::from("clean") }
}
impl CmdDesp for CleanCmd {
    fn cmd_info(&self) -> String { format!("Cmd,CleanCmd") }
    fn cmd_name(&self) -> String { CleanCmd::key() }
}
pub fn  cmd_regist(f : &mut CmdFatory)
{
    regist_cmd_creator::<ConfCmd>(f) ;
    regist_cmd_creator::<StartCmd>(f) ;
    regist_cmd_creator::<StopCmd>(f) ;
    regist_cmd_creator::<CleanCmd>(f) ;
}
