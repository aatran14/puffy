Trying to reimagine v1 of tpuf. I also know little about Rust. The impetus of this project is that I'm interested in learning Creusot becasue formal methods / verification is cool. Code written bespoke by yours truly. AI assisted for clarity, but not vibecoded as to entertain practice with Creusot.

As a corollary, I will use this proejct to take some field notes of what I think about Creusot/Rust.

1. quickly learned that creusot doesn’t have support for PartialOrd. It had a PR but it’s from 2023 and its latest comment was from 2025. So that was odd.

2. f32 giving me more trouble. it's hard to throw into a binary heap. Learned that making a wrapper struct for it was reasonable and not insane. I imagine rabitQ resolves this in v3. 

3. WAL time
