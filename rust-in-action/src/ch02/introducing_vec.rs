pub fn run() {
    let context_lines = 1;
    let target = "same";
    let text = "\
Every face, every shop,
bedroom window, public-house, and
dark square is a picture
feverishly turned--in search of what?
It is the same with books.
What do we seek
through millions of pages?";

    let mut tags: Vec<usize> = Vec::new();
    let mut ctx: Vec<Vec<(usize, String)>> = Vec::new();
    for (i, line) in text.lines().enumerate() {
        if line.contains(target) {
            tags.push(i);
            let v = Vec::with_capacity(2 * context_lines + 1);
            ctx.push(v);
        }
    }

    if tags.is_empty() {
        return;
    }

    for (i, line) in text.lines().enumerate() {
        println!("{},{}", i, line);
        for (j, tag) in tags.iter().enumerate() {
            println!("{},{}", j, tag);
            let lower_bound = tag.saturating_sub(context_lines);
            let upper_bound = tag + context_lines;
            println!("{},{}", lower_bound, upper_bound);

            if (i >= lower_bound) && (i <= upper_bound) {
                let line_as_string = String::from(line);
                let local_ctx = (i, line_as_string);
                ctx[j].push(local_ctx)
            }
        }
    }

    for local_ctx in ctx.iter() {
        for &(i, ref line) in local_ctx.iter() {
            let line_num = i + 1;
            println!("{}: {}", line_num, line)
        }
    }
}
