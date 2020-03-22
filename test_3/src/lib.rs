/*#![feature(register_tool)]
#![feature(stmt_expr_attributes)]
#![register_tool(hir_syn)]*/

#[macro_use]
extern crate serde;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod dtos {
    #[derive(Serialize, Deserialize, Debug,Clone)]
    pub struct NewAction {
        pub url: String,
    }
    
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub enum ClientMsg {
        NewAction(NewAction),
        ResendAllStatuses,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub enum ServerMsg {
        StatusUpdate(StatusInfo),
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct StatusInfo {
        pub url: String,
        pub name: Option<String>,
        pub status: Status,
        pub size: Option<usize>,
    }
    #[derive(Serialize, Deserialize, Debug, Clone, strum_macros::AsRefStr)]
    pub enum Status {
        Pending,
        Init,
        Progress(usize),
        Error(String),
        Finished(Option<String>),
        TreeUpdated,
    }
    pub const BIND_ADDR: &'static str = "127.0.0.1:2795";
}

