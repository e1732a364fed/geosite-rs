use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    // 读取二进制文件
    let mut file = File::open("geosite.dat")?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    // 解析 Protobuf
    let proto_list = geosite_rs::read(&buf).expect("load ok");

    // 打印解析结果
    for site in proto_list.site_group {
        println!("Country: {}", site.tag);
        for domain in site.domain {
            println!("  - Type: {:?}, Value: {}", domain.r#type(), domain.value);
        }
    }

    Ok(())
}
