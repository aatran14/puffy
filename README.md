<blockquote>

- tell me what you know

- tell me what you don't know

- then tell me what you think

- always distinguish which from which

</blockquote> 

- former secretary of state, colin powell


---

In short, this is a reimagination of v1 tpuf.

I know little about Rust. The impetus of this project is that I'm interested in learning Creusot becasue formal methods / verification is truly important. Writen bespoke by yours truly. AI assisted for clarity, but not vibecoded as to entertain practice with Creusot. 

This sounds counterinuitive to modern agentic advice. The assymetry lives in the fact that entropy erodes everything, even software. So while agentic workflows live by rebirthing itself, databases on the otherhand live by withstanding the entropy. Take CockroachDB for example. CockroachDB is aptly named because cockroaches don't die.

This is a remarkable result. It means a database moves fast because she can prove the most truths (with pace). If this was not true, then a human could spin up a database architecture agentically overnight and be competitive. But we know this is not true because the opposite phenomenon occurs. The half-basked DB doens't just die. It gets eaten alive; Agar.io-style.

---

In undergrad, I studied Formal Methods wrto model checking over transition systems. Problems typically involved using linear temporal logic (LTL) to compute policies that achieve desired properties. Two prevalent ones that come to mind are: reachability and unreachability. Reachability is when the system must reach a target state. Enforcing unreachbility means never reach a bad state. You can imagine that doing this might be useful in a game-theoretic setting, or for an autonomous robot. In finite worlds, you can do this naively by working backwards from target states. This means it even applies to non-deterministic systems.

For large state spaces, this naive method breaks down, but you at least know the properties exist. There are also infinite state systems. We call this thinking `Büchi Automata`, which operate on infinite words. Think of this as systems that do not aim to terminate. In literature, we discuss  their properties as `Büchi condition`. Thinking in infinite worlds has a lot of consequences, but are important in sympathizing with the finite state results.

To add to my chagrin on this topic, I never bothered to look at the other flavor of formal methods called `deductive verification`. This flavor is where Creusot lives. Think of it as mathematical contracts about code (or even statements).

As a corollary, I will use this investigation to take some field notes of what I think about Creusot/Rust.

1. Quickly learned that creusot doesn’t have support for PartialOrd. It had a PR but it’s from 2023 and its latest comment was from 2025. So that was odd.

2. f32 giving me more trouble. it's hard to throw into a binary heap. Learned that making a wrapper struct for it was reasonable and not insane. I imagine rabitQ resolves via quantization.

3. The intent of the WAL is so pure and pristine.

4. Still don't know what `usize` does.

5. Always use `UUID`s.

6. At around 300 loc, my mental model of the variables and signatures experienced breakdown. It didn't harm the high level overview, but I was losing all sense of &mut. Then I saw Creusot's `*buf` / `^buf` notation.

7. 