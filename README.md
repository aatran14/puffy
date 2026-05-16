<blockquote>

- tell me what you know

- tell me what you don't know

- then tell me what you think

- always distinguish which from which

from former secretary state colin powell
</blockquote>

---

In short, this is a reimagination of v1 tpuf.

I know little about Rust. The impetus of this project is that I'm interested in learning Creusot becasue formal methods / verification is truly important. Writen bespoke by yours truly. AI assisted for clarity, but not vibecoded as to entertain practice with Creusot. 

This sounds counterinuitive to modern agentic advice. The assymetry lives in the fact that entropy erodes everything, even software. So while agentic workflows live by rebirthing itself, databases on the otherhand live by withstanding the entropy. CockroachDB is aptly named. Cockroaches don't die. 

This is a remarkable result. It means that a database "runs" fast solely because it can prove the most truths (with pace). If this was not true, then a human could spin up a database architecture agentically. But we know this is not true because the opposite phenomenon occurs. The half-baked db doesn't just die. It gets eaten alive. Agar.io-style.

In undergrad, I studied Formal Methods wrto model checking over transition systems. Solving problems typically involved using linear temporal logic (LTL) to compute policies that achieve desired properties. Two prevalent ones that come to mind are: reachability and unreachability. Reachability is when the system must reach a target state. Enforcing unreachbility means never reach a bad state. You can imagine that doing this might be useful in a game-theoretic setting, or for an autonomous robot. In finite worlds, you can do this naively by working backwards from target states. This means it even applies to non-deterministic systems.

For large state spaces, this naive method breaks down, but you at least know the properties exist. There are also infinite state systems. We call this thinking `Büchi Automata`. In literature, we discuss the Büchi condition because Büchi automata operate on infinite words, meaning the reasoning applies to systems that do not terminate. This has a lot of consequences, but are important in sympathizing with the finite state results.

To add to my chagrin on this topic, I never bothered to look at the other flavor of formal methods called `deductive verification`. This flavor is where Creusot lives. Think of it as mathematical contracts about code (or even statements). 

Creusot lives in another flavor of formal methods called deductive verification. Think of them as mathematical contracts about the code.

As a corollary, I will use this proejct to take some field notes of what I think about Creusot/Rust.

1. Quickly learned that creusot doesn’t have support for PartialOrd. It had a PR but it’s from 2023 and its latest comment was from 2025. So that was odd.

2. f32 giving me more trouble. it's hard to throw into a binary heap. Learned that making a wrapper struct for it was reasonable and not insane. I imagine rabitQ resolves this in v3. 

3. The intent of the WAL is so pure and pristine.

4. Still don't know what `usize` does.

5. Always use `UUID`s.

6. At around 300 loc, my mental model of the variables and signatures experienced breakdown. It didn't harm the high level overview, but I was losing all sense of &mut. Then I saw Creusot's `*buf` / `^buf` notation.

7. 