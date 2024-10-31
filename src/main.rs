use std::{ env, fs, process };
use libphext::phext;

// ├ └ ─ │

// 1.1.1/1.1.1/1.1.1

/*
┌─ Phext Document
├─ Library 1
  ├─ Shelf 1
    ├─ Series 1
      ├─ Collection 1
        ├─ Volume 1
          ├─ Book 1
            ├─ Chapter 1
              ├─ Section 1
                └─ Scroll 1: Content here
*/

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        process::exit(1);
    }

    let phext_raw = fs::read_to_string(&args[1]).unwrap();
    let scrolls = phext::phokenize(&phext_raw);

    let mut dimension_widths: [usize; 9] = [1; 9];
    for scroll in &scrolls {
        dimension_widths[0] = dimension_widths[0].max(scroll.coord.z.library.to_string().len());
        dimension_widths[1] = dimension_widths[1].max(scroll.coord.z.shelf.to_string().len());
        dimension_widths[2] = dimension_widths[2].max(scroll.coord.z.series.to_string().len());
        dimension_widths[3] = dimension_widths[3].max(scroll.coord.y.collection.to_string().len());
        dimension_widths[4] = dimension_widths[4].max(scroll.coord.y.volume.to_string().len());
        dimension_widths[5] = dimension_widths[5].max(scroll.coord.y.book.to_string().len());
        dimension_widths[6] = dimension_widths[6].max(scroll.coord.x.chapter.to_string().len());
        dimension_widths[7] = dimension_widths[7].max(scroll.coord.x.section.to_string().len());
        dimension_widths[8] = dimension_widths[8].max(scroll.coord.x.scroll.to_string().len());
    }

    println!("┌─ {}", args[1]);

    let mut prev_coord = phext::default_coordinate();
    for (i, scroll) in scrolls.iter().enumerate() {
        let coord = &scroll.coord;
        let is_last = i == scrolls.len() - 1;
        let connector = if is_last { "└─" } else { "├─" };
        
        if i == 0 || coord.z.library != prev_coord.z.library {
            println!("{} Library {}", connector, coord.z.library);
        }

        if i == 0 || coord.z.shelf != prev_coord.z.shelf {
            println!("  {} Shelf {}", connector, coord.z.shelf);
        }

        if i == 0 || coord.z.series != prev_coord.z.series {
            println!("    {} Series {}", connector, coord.z.series);
        }

        if i == 0 || coord.y.collection != prev_coord.y.collection {
            println!("      {} Collection {}", connector, coord.y.collection);
        }

        if i == 0 || coord.y.volume != prev_coord.y.volume {
            println!("        {} Volume {}", connector, coord.y.volume);
        }

        if i == 0 || coord.y.book != prev_coord.y.book {
            println!("          {} Book {}", connector, coord.y.book);
        }

        if i == 0 || coord.x.chapter != prev_coord.x.chapter {
            println!("            {} Chapter {}", connector, coord.x.chapter);
        }

        if i == 0 || coord.x.section != prev_coord.x.section {
            println!("              {} Section {}", connector, coord.x.section);
        }

        println!("                {} Scroll {}: {}", connector, coord.x.scroll, coord);
        prev_coord = *coord;
    }
}

fn print_aligned(content: &str, width: usize) {
    let padding = " ".repeat(width - content.len());
    println!("{}{}", padding, content);
}
