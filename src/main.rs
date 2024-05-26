use std::{collections::VecDeque, env, process::exit};

// quote a string to single character wrapped string that won't have any expansions in it
// note might not work in all cases correctly with newlines and stuff
fn quote_kakoune(string: &str) -> String {
    format!("'{}'", string.replace('\'', "''"))
}

// print a command to output an error in kakoune
// fn kak_fail(error: &str) {
//     println!("fail {}", quote_kakoune(error))
// }

fn fail_with(error: &str) -> ! {
    kak_debug(error);
    exit(1)
}

fn kak_debug(text: &str) {
    eprintln!("{}", quote_kakoune(text));
}

type Location = (usize, usize);

type Selection = (Location, Location);

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        fail_with("not enough arguments passed to reasymotion")
    }


    match args[1].as_str() {
        "start" => {
            let selection_locations = match env::var("kak_selections_desc") {
                Ok(a) => a,
                Err(err) => fail_with(&format!("failed to get selection locations error: {}", err)),
            };

            let easymotion_keys = match env::var("kak_opt_reasymotion_keys") {
                Ok(a) => a,
                Err(err) => fail_with(&format!("failed to get option reasymotion-keys: {}", err)),

            };

            let selection_locations = match selection_locations
                .split(' ')
                .map(|selection| -> Option<Selection> { parse_selection(selection) })
                .collect::<Option<Vec<Selection>>>()
            {
                Some(a) => a,
                None => fail_with("failed to parse selection_locations"),
            };

            // todo get option for what keys to use
            let keystrokemap: Vec<(Selection, String)> =
                generate_keysequences(selection_locations, &easymotion_keys).collect();

            generate_highlighting(&keystrokemap);

            generate_on_key_func(&keystrokemap)
        }
        "keypress" => {
            if args.len() <= 3 {
                fail_with("not enough arugments to reasymotion keypress command");
            }
            let args = &args[2..];

            let selections = match args
                .chunks(2)
                .map(|a| match a {
                    [label, selection] => {
                        let selection = parse_selection(selection)?;

                        Some((selection, label.clone()))
                    }
                    _ => fail_with("no matching selection for final label"),
                })
                .collect::<Option<Vec<(Selection, String)>>>()
            {
                Some(a) => a,
                None => fail_with("failed to parse arguments to reasymotion keypress"),
            };

            let keypressed = match env::var("kak_key") {
                Ok(a) => a,
                Err(err) => fail_with(&format!("failed to get selection locations error: {}", err)),
            };

            let cancel_selection = selections[0].clone();

            let remaining:Vec<(Selection,String)> = selections.into_iter().filter_map(
                |(selection,label)|
                {
                    let new_label = label.strip_prefix(&keypressed)?;
                    Some((selection,new_label.to_string()))

                }).collect();

            match remaining.len() {
                0 => {
                    set_selection(cancel_selection.0);
                    remove_highlighting();
                },
                1 => {
                    let selection = &remaining[0];
                    set_selection(selection.0);
                    remove_highlighting();
                },
                _ => {
                    generate_highlighting(&remaining);
                    generate_on_key_func(&remaining);
                },
            }

            // let cancel_location =
        }

        _ => {
            fail_with("that command is currently unsupported");
        }
    }

    // generate call to it so that the next key is fed back into this executable via
    // another call with all the information about the remaining selections passed in
    // as keypress (SelectionKey(stored as normal escaped string) x1.y1,x2.y2)*
    //
    // the key is fed back in as an the env variable
}

fn set_selection(selection: ((usize, usize), (usize, usize))) {
    println!("select {}.{},{}.{}", selection.0.0,selection.0.1,selection.1.0,selection.1.1);
}

fn remove_highlighting() {
    println!("remove-highlighter buffer/reasymotionselections");
    println!("remove-highlighter buffer/reasymotionbackground");
}

fn generate_on_key_func(keystrokemap: &[(Selection, String)]) {
    let arguments = keystrokemap
        .iter()
        .map(|(selection, label)| {
            format!(
                "{label:?} {}.{},{}.{}",
                selection.0 .0, selection.0 .1, selection.1 .0, selection.1 .1
            )
        })
        .collect::<Vec<_>>()
        .join(" ");

    // commend with env var name provided to make sure
    // that it is passed into the env of the command
    println!("on-key {}",quote_kakoune(&format!("evaluate-commands %sh{{ # $kak_key \n rkak_easymotion keypress {arguments} }}")));
}

fn format_highlight(selection: &Selection, label: &String) -> String {
    let start = selection.0;

    let length = label.len().min(min_selection_length(selection));

    quote_kakoune(&format!(
        "{}.{}+{length}|{{REasymotionForeground}}{}",
        start.0, start.1,&label[0..length.max(1)]
    ))
}

fn min_selection_length(selection: &Selection) -> usize {
    eprintln!("selection that getting length: {:?}",selection);
    if selection.0.0 != selection.1.0 {
        0
    } else {
        selection.1.1 - selection.0.1
    }
}

// prints everything
fn generate_highlighting(keystrokemap: &[(Selection, String)]) {
    let labels = keystrokemap
        .iter()
        .map(|(selection, label)| format_highlight(selection, label))
        .collect::<Vec<_>>()
        .join(" ");

    println!("set-option buffer reasymotionselections %val{{timestamp}} {labels}")
}

// generate a mapping between a selection and it's key sequence
fn generate_keysequences(
    selection_locations: Vec<Selection>,
    keys: &str,
) -> impl Iterator<Item = (Selection, String)> {
    // allocate enough that the vector should never have to be copied
    let mut key_sequence_queue: VecDeque<String> =
        VecDeque::with_capacity(selection_locations.len() + keys.len() - 1);

    if keys.is_empty() {
        fail_with("easymotion keys has a length of 0")
    }

    for c in keys.chars() {
        key_sequence_queue.push_back(c.to_string());
    }

    while key_sequence_queue.len() < selection_locations.len() {
        let a = key_sequence_queue
            .pop_front()
            .expect("characters should have been added");
        for c in keys.chars() {
            let mut base = a.clone();
            base.push(c);
            key_sequence_queue.push_back(base);
        }
    }

    selection_locations.into_iter().map(move |selection| {
        let a = key_sequence_queue
            .pop_front()
            .expect("condition on while loop");
        (selection, a)
    })
}

fn parse_selection(selection: &str) -> Option<Selection> {
    let mut locs = selection.split(',');
    let first = locs.next()?;
    let second = locs.next()?;
    let first = parse_location(first)?;
    let second = parse_location(second)?;
    Some((first, second))
}

fn parse_location(location: &str) -> Option<Location> {
    let mut location = location.split('.');
    let a = location.next()?;
    let x = a.parse::<usize>().ok()?;
    let y = location.next()?.parse::<usize>().ok()?;
    Some((x, y))
}
