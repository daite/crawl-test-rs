use select::document::Document;
use select::predicate::{Class, Name};

fn main() {
   let r = get_data();
   for (k, v) in r {
       println!("{} => {}", k, v);
   }
   get_magnet();
}

fn get_data() -> Vec<(String, String)>{
    let site_url = "https://torrentjuju.com/bbs/";
    let document = Document::from(include_str!("../bbs.html"));
    let mut v = vec![];
    for node in document.find(Class("media-heading")){
        let title = node.text().replace("\n", "");
        let bbs_link = 
            node.find(Name("a"))
                .next()
                .unwrap()
                .attr("href")
                .unwrap();
        let bbs_link = format!("{}{}", site_url, bbs_link.replace("./", ""));
        v.push((title, bbs_link));
    }
    v
}

fn get_magnet() -> String {
    let document = Document::from(include_str!("../magnet.html"));
    let magnet = document.find(Class("list-group-item")).skip(1)
                 .next()
                 .unwrap()
                 .find(Name("a"))
                 .next()
                 .unwrap()
                 .attr("href")
                 .unwrap();
    magnet.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_data_func_test() {
        let data = get_data();
        let sample = &data[0];
        assert_eq!(
            "동상이몽2 너는 내 운명.E235.220228.720p-NEXT.mp4",
            sample.0,
        );
        assert_eq!(
            "https://torrentjuju.com/bbs/board.php?bo_table=enter&wr_id=32183",
            sample.1,
        );
    }
    #[test]
    fn get_magnet_func_test(){
        let m = get_magnet();
        assert_eq!(
            "magnet:?xt=urn:btih:04a6888916168f67e7f16cafb55fcbcfef7317e2",
            m
        );
    }

}
