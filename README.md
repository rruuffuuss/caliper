# Caliper

Caliper is a very fast but (for now) very dumb rule checking engine.
Rather than parsing a document and then evaluating rules against it, caliper builds a set of rules into trie based finite state machine, which can then be run against any document in a single pass. This is not a novel idea (to my unfortunate suprise), and (YFilter)[https://yfilter.cs.berkeley.edu/doc/yfilter-tods-2003.pdf?] has been a useful source in my research.

In tandem with the single pass FSM based rule evaluation, Caliper will investigate
- improving document streaming performance using SIMD instructions and minimal XML/Json tokenisation
- compiling rules to native machine code using LLVM to be directly executed on document bytes

I became interested rule engines for payment processing (new high volume instant payment schemes have second level SLAs), but this implementation is academic and focuses on speed over useability. It is unlikely this will ever support complete XPath or complete JsonPath.
