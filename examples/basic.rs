use baidu_lac_rs;

fn main() {
    let query = "LAC是个优秀的分词工具";
    let result = baidu_lac_rs::run(query);
    println!("result {:#?}", result);

    let query = "百度是一家高科技公司";
    let result = baidu_lac_rs::run(query);
    println!("result {:#?}", result);

}
