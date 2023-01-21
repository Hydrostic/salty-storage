use log::info;
use rbatis::Rbatis;
use rbdc_mysql::driver::MysqlDriver;

pub struct Instance {
    pub engine: Rbatis,
}

impl Instance {
    pub async fn init() -> Self {
        let rb = Rbatis::new();
        rb.init(MysqlDriver {}, "mysql://test:114514@localhost:3306/salty")
            .unwrap();
        // rb.log_plugin.set_level_filter(log::LevelFilter::Error);
        let version: String = rb.fetch_decode("select version();", vec![]).await.unwrap();
        info!("Mysql version: {}", version);
        Instance { engine: rb }
    }
}
