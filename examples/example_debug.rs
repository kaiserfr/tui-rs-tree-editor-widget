use std::error::Error;
use std::io;

use tui_tree_widget::{Tree, TreeItem, TreeState};

#[derive(Debug)]
struct App<'a> {
    state: TreeState<&'static str>,
    items: Vec<TreeItem<'a, &'static str>>,
}

impl<'a> App<'a> {
    fn new() -> Self {
        Self {
            state: TreeState::default(),
            items: vec![TreeItem::new_leaf(String::from("a"), "Alfa")],
        }
    }
}

// impl<'a> App<'a> {
//     fn new() -> Self {
//         Self {
//             state: TreeState::default(),
//             items: vec![
//                 TreeItem::new_leaf("a", "Alfa"),
//                 TreeItem::new(
//                     "b",
//                     "Bravo",
//                     vec![
//                         TreeItem::new_leaf("c", "Charlie"),
//                         TreeItem::new(
//                             "d",
//                             "Delta",
//                             vec![
//                                 TreeItem::new_leaf("e", "Echo"),
//                                 TreeItem::new_leaf("f", "Foxtrot"),
//                             ],
//                         )
//                         .expect("all item identifiers are unique"),
//                         TreeItem::new_leaf("g", "Golf"),
//                     ],
//                 )
//                 .expect("all item identifiers are unique"),
//                 TreeItem::new_leaf("h", "Hotel"),
//                 TreeItem::new(
//                     "i",
//                     "India",
//                     vec![
//                         TreeItem::new_leaf("j", "Juliett"),
//                         TreeItem::new_leaf("k", "Kilo"),
//                         TreeItem::new_leaf("l", "Lima"),
//                         TreeItem::new_leaf("m", "Mike"),
//                         TreeItem::new_leaf("n", "November"),
//                     ],
//                 )
//                 .expect("all item identifiers are unique"),
//                 TreeItem::new_leaf("o", "Oscar"),
//             ],
//         }
//     }
// }

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::new();

    // let item = &app.items[1].children()[1];
    let item = &app.items[1];

    item.do_print();

    // print!("{:#?}", item);
    // print!("{:#?}", app);

    Ok(())
}