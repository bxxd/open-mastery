# EdTech Landscape Analysis

Date: 2026-03-11

Competitive landscape research across consumer AI education products. Sources: company websites, earnings transcripts (NRDY Q4 2025, MH Q2 FY2026), Reddit user reviews (Alpha Anywhere thread, ~20 comments over 6 months).

---

## The Players

| Platform | Price | Age | AI Role | Quality Signal |
|---|---|---|---|---|
| Khan Academy | Free/$44yr | All | GPT-4 Socratic tutor | Ubiquitous, proven |
| Math Academy | $49/mo | 4th-university | Adaptive knowledge graph | WASC, 4x speed claims |
| Synthesis | $25-45/mo | 5-11 | Hybrid AI+neuroscience | SpaceX origin, neurodiverse focus |
| AoPS | ~$200-400/course | 5-12 | None | IMO champions, 30yr track record |
| Alpha Anywhere | $833/mo | K-12 | Full curriculum AI | Buggy, IXL terminated contract |
| NRDY (Varsity Tutors) | ~$364/mo | All | AI-augmented human tutors | Just hit EBITDA+, 33K members |
| MH (McGraw-Hill) | Institutional | All | ALEKS adaptive + AI tools | $2B rev, 30% Higher Ed share |

### Math Academy ($49/mo)
- AI-adaptive math, K-12 through university (Linear Algebra, DiffEq, ML math)
- "Fractional Implicit Repetition" knowledge graph + cognitive science. NOT LLM-based
- Claims 4x speed: "6th grade to calculus in <1 year." 180 classroom hours -> 20-40 hours
- WASC accredited (same body as Stanford, UCLA)
- Closest architectural analog to Open Mastery's graph approach

### Synthesis Tutor ($25-45/mo, $999 lifetime)
- AI math tutor ages 5-11, multisensory/interactive
- Origin: SpaceX school (Josh Dahn built Elon's experimental school, 2014)
- Explicitly "not ChatGPT" — hybrid AI + educator/neuroscientist design
- Strong neurodiverse support (dyslexia, ADHD, ASD, 2E)
- 25K+ families
- Lifetime pricing ($999-1,499) is a churn confession: LTV < $1,000

### AoPS (Art of Problem Solving)
- Advanced math for competition/elite track, grades 5-12. Since 1993
- All 6 USA IMO 2024 team members were AoPS alumni
- NO AI. Human-designed curriculum, forums, peer collaboration
- WASC accredited. Founded by USAMO winner Richard Rusczyk
- Gold standard for gifted math kids. Not AI-native and may not need to be

### Alpha School / Alpha Anywhere
- Campus: $40-75K/yr. Anywhere: $833/mo
- "2-Hour Learning" — AI tutoring mornings, life skills/passion projects afternoons
- Founder: Joe Liemandt ($1B committed), backed by Maples/Lonsdale
- 12+ campuses, ~15 more Fall 2026. Cognia accredited
- Reddit reviews: buggy, IXL + Khan reskin, sporadic support. IXL terminated contract
- Afternoon program (running Airbnbs, food trucks, musicals) is actually more interesting than the AI morning

### NRDY / Varsity Tutors (Public: NRDY)
- Q4 2025: Revenue $49.1M (+2% YoY). First positive EBITDA: $1.3M (vs -$5.5M prior year)
- 33,200 active members, ARPAM $364 (+21% YoY). Headcount down 22% (AI automation)
- Completely rewrote platform in 2025 to AI-native codebase
- Explicitly anti-pure-AI. CEO: "Ask parent learn human tutor AI, all provide same answer — human tutor"
- Betting human tutoring is durable. AI makes tutors more effective, not replaceable
- "Moments of learning" — parents see exact camera frame where kid gets it

### McGraw-Hill (Public: MH)
- Q2 FY2026: Revenue $669M. Digital $352M (+7.6%), 53% of total
- Higher Ed grew 14% YoY. Market share: 30% (was 21% a decade ago)
- Adjusted EBITDA $286M, 43% margin. $1.9B RPO
- AI products: ALEKS (adaptive math, 20% pass rate improvement at Clemson), AI Reader (20M+ interactions), Sharpen Advantage, Writing Assistant, Clinical Reasoning, Teacher Assistant
- CEO quote: "Large language models demand structured learning progression and continuous student interaction data for true comprehension over memorization"
- California Math opportunity FY2027 (+$300M TAM), Florida ELA FY2028

---

## What The Market Understands That Customers Never Say

The kid is not the customer. The parent's shame is.

Every Reddit commenter is in pain: "I'm at a loss with my 5th grader. He has given up." "He couldn't read multi-syllable words in 4th grade." "She's struggling in public school." Nobody walks in saying "I need a better knowledge graph." They walk in saying "something is wrong and I think it might be my fault."

1. **Selling absolution, not education.** "2-hour learning" and "4x speed" say: your kid was never broken, the system was. Reframes parent from failure to rescuer.

2. **Removing the parent from the conflict loop.** Happiest Alpha review: "Thank God I don't have to nudge him." Every fight over homework is a micro-rupture in the parent-child relationship. The product that wins stops the kitchen table from becoming a battlefield.

3. **Dashboards are anxiety medication.** Every positive review mentions seeing progress daily/weekly/monthly. NRDY's "moments of learning" camera clips. The dashboard doesn't help the kid learn. It lets the parent sleep.

4. **"Mastery" is a proxy for control.** Parents can't control a classroom. Mastery percentages give them a legible metric. "90% mastery required" is a promise that someone is watching.

5. **Willingness to pay correlates with desperation, not sophistication.** Alpha at $833/mo and NRDY at $350/mo are last-resort products. These parents don't comparison shop.

6. **The kids' problem is emotional, not cognitive.** "He has given up and refuses to do any work." That's learned helplessness, not a math gap. Alpha gets it crudely (reward system). AoPS doesn't address it (selects for already-motivated). The graph isn't teaching better — it's removing the human from the shame loop.

---

## Three Assumptions The Market Is Built On

### Assumption 1: Learning is an individual optimization problem

Every player assumes the unit of learning is one kid, one screen, one problem. The entire market optimizes that loop.

**Wrong if:** Kids learn more from arguing with each other than from solving problems alone. AoPS forums hint at this — community as important as curriculum, IMO champions came through peer competition. If collaborative cognition research (CSCL) is right, the optimal loop is student->student->problem->argument->insight. Every product is missing the most important node.

### Assumption 2: The knowledge graph is the territory

Math Academy, ALEKS, Open Mastery — all assume math decomposes into a DAG of prerequisites, and traversing the DAG in order IS learning.

**Wrong if:** Understanding is a phase transition, not a sequence. Kids fumble through fractions in fog until something clicks and the whole structure reorganizes at once. The prerequisite graph is a useful lie — it sequences content but doesn't model cognition. Evidence: transfer failure. Kids who "master" fraction arithmetic on adaptive platforms routinely fail to apply fractions in novel contexts (measurement, probability, ratio reasoning). The graph says they know it. They don't.

### Assumption 3: Parents are the buyer forever

The entire market prices, markets, and designs for the parent's anxiety and credit card.

**Wrong if:** The generation on adaptive platforms develops learning agency earlier and rejects parental mediation. A 10-year-old asks Claude to teach them calculus. No platform, no subscription, no dashboard. The SaaS model assumes a gatekeeper between learner and learning. If the gatekeeper becomes unnecessary, the winning product is B2C-to-kid — free or nearly free, learner-sovereign, which means open source.

---

## The Real Problem

Every product answers: "How does my kid learn math faster?"

The parent's actual question: "What does my kid need to become so they're not obsolete?"

Completely different questions. The first has a product. The second has no product because nobody knows the answer — and companies that pretend to know are selling the same curriculum faster, which is optimizing horse-breeding in 1905.

What each company does with the real anxiety:

- **Math Academy:** "Learn math 4x faster." But if AI does math, speed is irrelevant. Training the horse to run faster.
- **Alpha School:** "2 hours academics, afternoons for life skills." Closest to addressing it — the life skills half (Airbnb, food truck) is more valuable than the AI tutoring half. They have the pieces backwards.
- **Khan/MH/NRDY:** Optimizing delivery of existing curriculum. The curriculum is the part parents are worried about.
- **AoPS:** The only one that accidentally answers the real question. They don't teach math. They teach how to struggle with something hard and not quit. The math is the medium, not the message. Their kids become mathematicians because AoPS taught them to love being stuck.

### The Three Things AI Cannot Replace

1. **The capacity to want something and pursue it** (Berridge's WANTING — dopamine is pursuit, not arrival)
2. **The ability to struggle at the edge of competence** (Csikszentmihalyi's flow — requires real friction, not adaptive difficulty that keeps you comfortable)
3. **The capacity to be wrong in front of others and survive it** (Vygotsky's social crucible — the AoPS model)

No AI can want FOR you. No AI can struggle FOR you. No AI can be embarrassed FOR you. These are embodied, chemical, irreducibly human capacities. The gamified, personalized, always-encouraging AI tutor is training them out of children.

The $20B edtech market builds products that make anxiety feel managed while systematically removing the friction that develops the only skills that will matter.

---

## Investor Kill Questions

### Q1: "Show me your moat. What stops me from building this in 6 months with Claude and $2M?"

The evidence says you can. Alpha Anywhere is basically "IXL + Khan" and IXL terminated their contract. Synthesis relies on "expert educators and neuroscientists" — human curation, the thing that gets automated. The only defensible assets: AoPS's 30 years of community, MH's 137 years of content + 30% market share. In consumer edtech, there is no moat. Switching cost is one parent Google search.

### Q2: "What does churn look like after month 3?"

Nobody discloses, which is the answer. NRDY emphasizes "higher retention in newer cohorts" (older cohorts churned badly). Synthesis's $999 lifetime option = LTV < $1,000. Alpha Anywhere: two weeks off for Christmas "has taken us some time to recover from." Every signal says 3-6 month engagement cliff. Dashboards and gamification are churn-prevention tools, not learning tools.

### Q3: "Your TAM is $20 billion. How much is real?"

NRDY: $20B claimed TAM, 33,200 members, $145M run rate = 0.7% penetration after years. Needed 22% headcount cuts to reach $1.3M EBITDA. MH reveals the real TAM: institutional buyers, $1.9B RPO, "90% of district revenue comes from local budgets." Consumer edtech TAM is a fiction. Real market is districts and universities. Consumer is a niche of affluent anxious parents — large enough for a small business, not a large one.

### Q4: "What happens when Khan's free AI tutor gets good enough?"

Khan is free. Khanmigo is $44/year. Math Academy charges 13x more. Alpha charges 227x more. The only justification is "our AI is better" — but the gap closes by the month. Math Academy's knowledge graph and AoPS's community are the only things that don't get eaten by better free AI. Everything else is on a countdown.

### Q5: "Show me evidence that any AI-tutored student outperformed a traditionally-taught student on anything other than a standardized test."

Silence. Math Academy claims "4x speed" — speed of completing curriculum, not understanding. Alpha claims "top 1% nationally" — on standardized tests, which measure exactly what adaptive AI optimizes for. MH cites Clemson "20% improvement in pass rates" — university exams, still structured assessments. Nobody has published evidence of transfer: using what you learned in a novel context. AoPS doesn't need to answer — IMO gold medals, MIT admissions, kids who chose mathematics. That's transfer. That's real.

The market optimizes for metrics (test scores, speed, mastery %) that may have no relationship to what parents think they're buying (a child who can think). Nobody runs the experiment because the answer might kill the business.

---

## The Central Tension

The product that works (AoPS: friction, failure, social risk) and the product that sells (Alpha: comfort, progress, anxiety relief) are in direct opposition.

Every design choice that develops desire in the child increases anxiety in the parent. Every design choice that relieves anxiety in the parent removes the friction that develops the child.

- Parent needs to see progress. Child needs to experience struggle.
- Parent needs safety. Child needs risk.
- Parent needs control. Child needs autonomy.

That's why nobody's built it. Not because they lacked insight. Because the insight contains a business model contradiction.

### The Principles Are Correct And Unbuildable

**Encounter requires surprise.** But engineered surprise is an oxymoron. The fifth time the graph shows "fractions connect to music," the kid recognizes the pattern: adults showing me connections. AoPS surprises emerge from problems hard enough that the solution path is genuinely unexpected. The surprise is IN the math. You can't graph that.

**Productive failure opposes the business model.** The parent pays. The parent needs anxiety relief. Deliberately frustrating their child, offering no progress bar, calling it a feature — every piece of evidence says the parent who sees their kid struggling cancels. NRDY rebuilt their platform to show "moments of learning" ON CAMERA because parents need to SEE progress. First month churn would be catastrophic.

**Audience means a social layer for children.** COPPA compliance, content moderation, bullying liability, predator risk. AoPS manages this because their community is tiny and self-selecting. Scaling a social layer for kids is a regulatory minefield that has killed companies with far more resources.

---

## The One Crack

AoPS barely markets to parents. The website is ugly. The onboarding is "here's a hard problem, good luck." Parents who sign up have ALREADY resolved their own anxiety — they've decided struggle is good, competition builds character, their kid can handle being wrong. AoPS doesn't sell anxiety relief. It sells to parents who don't need it.

Tiny market. Zero churn. The parent who believes in productive struggle doesn't cancel when their kid is frustrated. They say "good."

The question: is the number of parents who already believe struggle is good large enough to build a business on?

The evidence says it might be growing fast. The anxiety about AI making education obsolete is CREATING a new cohort of parents who've lost faith in the optimization model. The parent reading the Alpha Anywhere Reddit thread and thinking "this is all bullshit" is the customer. Not the parent who wants a better dashboard. The parent who's realized dashboards are the problem.

### The Synthesis

Alpha identified the right product (afternoon life-skills workshops) and built the wrong business (AI school). They need the broken AI morning to be a "school" for regulatory and parent-anxiety reasons. Without it, they're a $15K/year enrichment program. With it, they're a $75K/year "school."

The gap: Alpha's afternoon product at a real price point, with a morning product that actually works.

- A working knowledge graph (Math Academy quality) for academic sequencing
- A social-friction community (AoPS culture) for developing desire and grit
- The morning earns the "school" label and the parent's anxiety dollars
- The afternoon produces the actual human outcome

Alpha has vision and capital but a broken product. Math Academy has a working product but no vision beyond faster math. AoPS has the culture but won't scale.

**The customer:** Not the anxious parent shopping for dashboards. The skeptical parent who stopped believing in green progress bars. Different customer, different price, different retention. Much smaller market — but a market where churn is near zero because the parent and the product are aligned on what matters.

The product isn't "learn math faster." The product is "the AoPS model applied beyond math, for the parents who already get it." The graph is plumbing. The community is the product. The customer is the parent who stopped believing.

---

## The Actual Next Move

The three options presented — build the best morning, design the afternoon, find 10 families — are all too big.

**Option 1 is a trap.** 131 YAML nodes in a Rust engine vs. Math Academy's 5 years, WASC accreditation, a team of mathematicians, at $49/mo. You will not catch them on the morning. You'll spend 18 months building graph infrastructure and end up with an inferior version of something that costs $49. The graph is a commodity. This analysis said so three iterations in.

**Option 2 is the real thing but the wrong skillset.** Program design for children is a domain with credentialed experts, developmental psychology literature, decades of practice. Designing "social friction + real-world artifacts" for 8-year-olds isn't a code problem — it's an education problem. The people who do this well are former teachers, camp directors, Montessori practitioners. Not software engineers who had a sharp insight about Berridge and Vygotsky.

**Option 3 is honest but overscoped.** There's no product to run 10 families through. There's an insight.

### The move is smaller than all three options: find one family. Your own.

You are the customer you described — affluent, anxious about AI obsolescence, skeptical of dashboards, sophisticated enough to know green progress bars are theater.

- Put your kid on Math Academy for the morning ($49/mo)
- Design one afternoon per week yourself — a real project, a hard problem, another kid involved, no gamification, no rewards, just "here's something you can't do yet, want to try?"
- Run it for 3 months
- See what happens

That's not a business. It's a proof of concept with a sample size of one. But it's honest, costs almost nothing, and answers the question no analysis can: does a kid who gets Math Academy mornings plus friction-based afternoons develop differently than a kid who gets Math Academy alone?

If the answer is yes — visible in what the kid attempts, how they handle failure, whether they start choosing hard things — then you have something worth showing to the next 9 families.

If the answer is no, you saved yourself from building a company around a theory.

The entire edtech market is full of people who built the product before testing the thesis. Alpha built 12 campuses before their AI tutoring worked. NRDY rebuilt their platform twice before hitting breakeven. The luxury here is that the morning already exists (Math Academy, $49/mo) and the afternoon doesn't require software. It requires one adult, one kid, one hard problem, and the willingness to let the kid struggle.

Not code. Not YAML nodes. One kid, one afternoon, three months.
