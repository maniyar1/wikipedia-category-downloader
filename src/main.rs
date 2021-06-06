// importation syntax
use getopts::Options;
use rayon::prelude::*;
use select::document::Document;
use select::predicate::{Attr, Name, Predicate};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use url::Url;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("l", "levels", "how many category levels to go down", "1");
    opts.optflag(
        "a",
        "append-html",
        "append .html to the end of file names, will break links",
    );
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f.to_string())
        }
    };

    if matches.opt_present("h") || args.len() < 2 {
        print_usage(&program, opts);
        return;
    }

    let input = &args[1];
    let issue_list_url = Url::parse(input).unwrap();
    std::fs::create_dir_all("./wiki/").unwrap();

    if matches.opt_present("l") {
        if matches.opt_present("a") {
            get_category(
                issue_list_url.path(),
                matches.opt_get("l").unwrap().unwrap(),
                0,
                true,
            );
        } else {
            get_category(
                issue_list_url.path(),
                matches.opt_get("l").unwrap().unwrap(),
                0,
                false,
            );
        }
    } else {
        if matches.opt_present("a") {
            get_category(issue_list_url.path(), 1, 0, true);
        } else {
            get_category(issue_list_url.path(), 1, 0, false);
        }
    }
}

fn get_category(
    rel_url: &str,
    max_category_level: i32,
    current_category_level: i32,
    append_html: bool,
) {
    std::thread::sleep(std::time::Duration::from_millis(50)); // wikipedia is pretty cool, don't wanna spam em
    let url = "https://wikipedia.org/".to_owned() + rel_url;
    let resp = reqwest::blocking::get(&url).unwrap();
    assert!(resp.status().is_success());
    // Save file
    let body = resp.text().unwrap();
    let category_parsed_url = Url::parse(&url).unwrap();
    let segments = category_parsed_url
        .path_segments()
        .map(|c| c.collect::<Vec<_>>())
        .unwrap();
    let file_name = if append_html {
        format!("wiki/{}.html", segments[segments.len() - 1]) // Eg. https://en.wikipedia.org/wiki/Category:Libertarian_socialism -> wiki/Category:Libertarian_socialism.html
    } else {
        format!("wiki/{}", segments[segments.len() - 1]) // Eg. https://en.wikipedia.org/wiki/Category:Libertarian_socialism -> wiki/Category:Libertarian_socialism
    };
    let mut file = File::create(&file_name).unwrap();
    file.write_all(body.as_bytes()).unwrap();
    println!("{} Written", file_name);

    let document = Document::from(&body[..]);
    if current_category_level <= max_category_level {
        let subcats: Vec<&str> = document
            .find(Attr("id", "mw-subcategories").descendant(Name("a"))) // Get subcategories under <div id="mw-subcategories"> <a href="..."> </a> </div>
            .filter_map(|n| n.attr("href"))
            .collect();

        subcats
            .par_iter()
            .for_each(|x| get_category(x, max_category_level, current_category_level + 1, append_html));
    }

    let pages: Vec<&str> = document
        .find(Attr("id", "mw-pages").descendant(Name("a"))) // Get pages under <div id="mw-pages"> <a href="..."> </a> </div>
        .filter_map(|n| n.attr("href"))
        .collect();
    pages.par_iter()
        .for_each(|x| get_page(x, append_html));
}

fn get_page(rel_url: &str, append_html: bool) {
    std::thread::sleep(std::time::Duration::from_millis(50)); // wikipedia is pretty cool, don't wanna spam em
    let url = "https://wikipedia.org/".to_owned() + rel_url;
    let resp = reqwest::blocking::get(&url).unwrap();
    assert!(resp.status().is_success());
    let body = resp.text().unwrap();
    // Save file
    let category_parsed_url = Url::parse(&url).unwrap();
    let segments = category_parsed_url
        .path_segments()
        .map(|c| c.collect::<Vec<_>>())
        .unwrap();
    let file_name = if append_html {
        format!("wiki/{}.html", segments[segments.len() - 1]) // Eg. https://en.wikipedia.org/wiki/Category:Libertarian_socialism -> wiki/Category:Libertarian_socialism.html
    } else {
        format!("wiki/{}", segments[segments.len() - 1]) // Eg. https://en.wikipedia.org/wiki/Category:Libertarian_socialism -> wiki/Category:Libertarian_socialism
    };
    let mut file = File::create(&file_name).unwrap();
    file.write_all(body.as_bytes()).unwrap();
    println!("{} Written", file_name);
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} URL [options]", program);
    print!("{}", opts.usage(&brief));
}
