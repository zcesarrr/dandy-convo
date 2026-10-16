## Examples
```bash
dandy-convo gigi yatta
```
or from source
```bash
cargo run -- gigi yatta
```
output
```
 Gigi: Yatta, where do you like… stash all your candy?
Yatta: In my SUPER SECRET candy stash of course!
 Gigi: Riiiight, but like... where is it?
Yatta: Oh! It’s just to the left of- WAIT JUst a moment.
 Gigi: ...No really, go ahead.
Yatta: I'm not falling for THAT ONE!!!!
```

## Usage

```bash
cargo run -- [toon1_id] [toon2_id]
```

list all available toons
```bash
cargo run -- toons
```

options
```
--variant [num]     desc: display a different 
                    version of the conversation 
                    (only if it's available)

                    default: 1


--unformat          desc: remove indentation

                    default: false
```
