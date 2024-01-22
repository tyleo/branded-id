pub fn split_type_str(string: &str) -> String {
    let mut space_splits = string.split(' ').map(|space_split| {
        space_split.split(',').map(|comma_split| {
            comma_split.split('<').map(|left_chevron_split| {
                left_chevron_split
                    .split('>')
                    .map(|right_chevron_split| right_chevron_split.split("::"))
            })
        })
    });

    let mut result = String::new();

    let mut space_split = space_splits.next().unwrap();
    loop {
        let mut comma_split = space_split.next().unwrap();
        loop {
            let mut left_chevron_split = comma_split.next().unwrap();
            loop {
                let mut right_chevron_split = left_chevron_split.next().unwrap();
                loop {
                    let current_str = right_chevron_split.last().unwrap();
                    result.push_str(current_str);

                    if let Some(next_right_chevron_split) = left_chevron_split.next() {
                        result.push('>');
                        right_chevron_split = next_right_chevron_split;
                    } else {
                        break;
                    }
                }

                if let Some(next_left_chevron_split) = comma_split.next() {
                    result.push('<');
                    left_chevron_split = next_left_chevron_split;
                } else {
                    break;
                }
            }

            if let Some(next_comma_split) = space_split.next() {
                result.push(',');
                comma_split = next_comma_split;
            } else {
                break;
            };
        }

        if let Some(next_space_split) = space_splits.next() {
            result.push(' ');
            space_split = next_space_split;
        } else {
            break;
        };
    }

    result
}
