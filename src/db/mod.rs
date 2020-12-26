use rbatis::rbatis::Rbatis;
use rbatis::plugin::logic_delete::RbatisLogicDeletePlugin;

pub mod user;

lazy_static!{
    pub static ref RB: Rbatis = {
        let mut rbatis = Rbatis::new();
        // logic plugin
        rbatis.logic_plugin = Some(Box::new(RbatisLogicDeletePlugin::new_opt("del", 1, 0)));
        return rbatis;
    };
}

#[cfg(test)]
mod est {
   use serde_json::json;

   use crate::db::RB;

   #[async_std::test]
   async fn test_rbatis() {
    //    postgres://postgres:1234@localhost:5432/postgres mysql://root:123456@localhost:3306/test"
      RB.link("postgres://postgres:1234@localhost:5432/postgres").await.unwrap();
      let arg = &vec![json!(0)];
      let v: serde_json::Value = RB.fetch_prepare("", "CREATE TABLE t_user(

     );", arg).await.unwrap();
      println!("{}", v.to_string());
   }
}