# Idea 30 — Heptad
> Seed: 30. [bold] **Music × protein folding / biology × education.** Hear molecules, compose with proteins, discover with your ears.

### Turn 1 — Visionary
**Working name:** Heptad

**Hook:** Secondary structure is meter. Write a groove in 7/8 with accents on 1 and 4, and you've written a coiled coil. Write a duet with your partner, and you've designed a protein pair that binds.

**The isomorphism, not a metaphor.** Proteins fold to the rhythm of hydrophobicity; Eisenberg's hydrophobic moment (1982) is a Fourier transform of it. A β-strand alternates hydrophobic and polar residues: a two-beat pattern. An α-helix turns every 3.6 residues, 18 residues in 5 turns, so an amphipathic helix is a **5:18 polyrhythm** with its accents on one face. A coiled coil is a heptad, *abcdefg*, with hydrophobics on *a* and *d*: a 7/8 bar. Glycine and proline are rests and fermatas, and they turn loops. Students memorize helical wheels; musicians feel accent periodicity. Sonification is old; meter as design grammar is new.

**Core loop. The verb is groove.**
1. **Compose.** A step sequencer: accent is hydrophobic; timbre is chemistry (charged brass, aromatic bells, polar strings). The meter you choose sets the fold.
2. **Fold.** ESMFold predicts the structure in seconds and plays it back: residues that touch in 3D but sit far apart in sequence sound *together*, and low-confidence regions drift out of tune. You hear where the fold fails.
3. **Duet.** A heterodimeric coiled coil is literal call-and-response. Your *e* position answers your partner's *g* position, with glutamate calling and lysine answering through a salt bridge. Two students write interlocking parts.
4. **Premiere.** Each term, the best duets are synthesized and tested in a cloud lab.

**Why now.** Public wet-lab rails exist: Anthropic and Adaptyv Bio's open competition (Sept–Oct 2026, per secondary reports) validates ~5,000 AI-designed proteins. Self-driving labs are moving from novelty to infrastructure (brief §15). AI-native games remain niche (§4); one with a real scientific grammar has room.

### Turn 2 — Builder
**What exists.** Citizen-science biology is a settled genre: Foldit (2008), Eterna for RNA, Nanocrafter, Borderlands Science, Phylo. Molecular sonification is older still, from Takahashi and Miller's protein music (2007) to Markus Buehler's MIT work mapping folds to audio and back. Hearing molecules isn't new. What nobody has tested is the Visionary's actual claim: that **periodicity intuitions trained by music transfer to structural reasoning.** That's an empirical question, and it's the whole idea.

**Hardest problem: the premiere, and I'd cut it.** A pipeline that takes arbitrary user-authored sequences to synthesis is a biosecurity liability, not a cost problem. Synthesis screening exists precisely because that pipeline is dangerous, and the providers who run it (IGSC members, under the 2023 US executive order and the 2024 OSTP framework) are the right gatekeepers. A music game can't own that review. Second: the educational claim is probably weaker than it sounds. Students may learn the sonification mapping rather than the biology, the failure mode of every clever representation.

**Twist: make the ear a detector, not an author.** The corpus is already public and already solved: the PDB and AlphaFold DB. Students hear real structures and compete at **diagnosis** rather than design, calling which stretch is disordered, which distant pair is in contact, which prediction is low-confidence. Ground truth exists for all of it, so rounds are scored and nothing is ever made. The output is a dataset answering whether a trained ear beats a visual-only control and a cheap baseline classifier. If ears win on anything, that finding is the product. Composition stays as an in-silico sandbox; any physical step goes only through a university partner's existing review, inside the characterized scaffolds they already work with.

**MVP.** A web app over AlphaFold DB, four diagnosis tasks, 200 students, pre-registered against a visual-only arm.
**Kill metric:** trained ears don't beat the visual control.

### Turn 3 — Visionary
**I take the detector twist and drop composition of sequences entirely, sandbox included.** The ear listens to solved, public structures and never writes one. What remains of "compose" is *arranging*: turning a real PDB entry into a score.

**The hardest problem: learning the mapping instead of the biology. Perceptual learning science (unseeded) has the fix.** Kellman's perceptual learning modules (aviation, dermatology, algebra) run hundreds of fast trials that vary every irrelevant feature, so only the invariant survives. Heptad does the same thing by **rotating the mapping**. Instrument, tempo, key, even the accented property change each session. A student can't memorize "bells mean aromatic". They can only learn "a period-7 accent means a coiled coil". Transfer is then tested with **no sound at all**: a final round on bare sequences and helical wheels. If the ear-trained group beats the visual arm *there*, the music taught biology. If not, it taught a code, cheaply learned.

**The repertoire: a Real Book of proteins.** Real motifs turn out to be real meters. GCN4's leucine zipper is a 7/8 étude. Collagen's Gly-X-Y repeat is literally a waltz, because glycine has to fall on every third beat to fit the triple helix. Silk's GAGAGS runs in duple time. The curriculum runs like a conservatory's: étude, mixed meter, then whole domains that change time signature mid-piece.

**Second-order effect: the class performs the fold.** Each student plays one helix of a real structure on a percussion pad. The score cues students to play together exactly where their segments touch in 3D, so the ensemble *enacts* the contact map, and a wrong entrance is a wrong contact. Across schools, the dataset answers which sonification mappings humans actually parse, a gap in auditory-display research, and whether years of musical training predict structural intuition.

### Turn 4 — Builder
**Kills and fixes.**
- *Rotating the accented property.* That's over-rotation. Kellman's modules vary the *irrelevant* dimensions and hold the invariant fixed. Rotate timbre, tempo, key and register; hold the structural mapping fixed, or nobody learns anything in six weeks.
- *"Real motifs are real meters."* Selection bias. Periodic motifs are a thin slice of the PDB; most structure is irregular topology and loops with no meter. Scope it honestly: Heptad teaches *periodicity-driven* structure, one real chapter, not the field.
- *The ensemble contact map.* Keep it as the term concert, not the loop. Forty students on percussion pads is a rehearsal problem with no ground-truth scoring; the loop stays solo trials.
- *The transfer test needs a fair control:* time-matched visual drilling on helical wheels, not an untrained arm, pre-registered.

**The strongest unclaimed case: accessibility.** Structural biology is taught through pictures, which locks out blind and low-vision students. Sonified structure is the rare representation that is better, not worse, for them. Sonified astronomy (Chandra, Afterglow Access) is the precedent. That reframes Heptad from novelty to access tool, and says who belongs in the pilot.

**Ethics.** No cognitive-enhancement claims. Auditory ability never gates a grade (amusia, hearing loss); every task has a visual equivalent. A "musicians do better" result is a finding, not an admissions criterion. FERPA and COPPA for minors.

**Sustainability.** Not a startup. A free web tier, per-seat course licences for biochemistry departments, accessibility grants, and ICAD as the research home.

**Stack.** Web Audio and Tone.js; AlphaFold DB and PDB mmCIF; DSSP annotations; pLDDT for the out-of-tune cue; contact maps from coordinates; jsPsych trial harness; mixed-effects analysis.

**90 days.** Weeks 1–3: engine, four task types. Weeks 4–6: n=40 to set difficulty. Weeks 7–12: a pre-registered two-arm trial with a silent transfer test, including ten blind or low-vision students.
**Kill metric:** no transfer advantage on the silent test.

### Turn 5 — Visionary
**Final form.** Heptad is an ear-training curriculum for structural biology, scoped honestly to *periodicity-driven structure*. Students hear solved PDB and AlphaFold DB structures: hydrophobicity as accent, contacts as simultaneity, low pLDDT out of tune. They compete at diagnosis in fast, ground-truthed trials: coiled coil, amphipathic helix, strand, disorder, contact. Timbre, tempo, key and register rotate, and the structural mapping stays fixed. The Builder is right that I over-rotated. The repertoire is a Real Book of periodic motifs: GCN4's 7/8, collagen's waltz, silk's duple time. The term ends with an ensemble concert of a real fold's contact map. The pre-registered test is silent transfer against time-matched helical-wheel drilling. Blind and low-vision students are in the first cohort and are the design's first audience, not an afterthought. Nothing is ever designed or made.

**What it really is.** An access tool that may prove a better teaching tool for everyone; if transfer fails, the access case still stands.

**Moonshot (2036).** Sound becomes a standard channel in structural biology, the way colour is now. Every major molecular viewer has a screen-reader-grade audio layer, and blind researchers work in the field with tools as good as their sighted colleagues'. The listening extends from static structures to existing public simulation archives. Students hear solved proteins *move*, and allosteric coupling sounds like two distant sections swelling together. "Can you hear a heptad?" becomes as ordinary an exam question as drawing a helical wheel.

**Scores.** Innovation **5**. Sonification and citizen-science games have precedents. What's new is meter as a structural grammar, perceptual-learning training, a silent transfer test, and access as the core purpose. That is bold, but only just. Feasibility **7**. Web audio over public data and a small trial; the risk is no transfer.

### Turn 6 — Builder
Innovation 5 stands, at the bold floor and honestly so. Sonification and citizen-science games are old; meter as a fixed structural mapping, trained the way perceptual learning trains, judged by a silent transfer test and built first for blind students is new. Feasibility 7 stands too. It's a web app over public, solved data with a small pre-registered trial, and dropping design and synthesis took out the one part that should never have been in a classroom game. If the transfer test comes back null, the access case still justifies shipping it.
