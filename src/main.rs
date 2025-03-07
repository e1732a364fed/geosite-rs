use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    // 读取二进制文件
    let mut file = File::open("geosite.dat")?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    // 解析 Protobuf
    let proto_list = geosite_rs::decode_geosite(&buf).expect("load ok");

    // 打印解析结果
    for site in proto_list.entry {
        println!("Tag: {}", site.country_code);
        for domain in site.domain {
            println!("  - Type: {:?}, Value: {}", domain.r#type(), domain.value);
        }
    }
    // for group in proto_list.site_group {
    //     if group.tag == "GEOLOCATION-CN"{
    //     }
    //     if group.tag == "GEOLOCATION-!CN"{
    //     }
    // }

    Ok(())
}
