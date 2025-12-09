# On-Chain News Verification Network

**Project Name:** Zander

**Value Proposition:**

Zander gives people a way to verify news transparently and without relying on any single organization to decide what's true. By putting news and it's verification process on-chain, anyone can see how conclusions were reached. In a world where AI generated misleading content is growing fater than traditional fact checkers can react, this platform offers a comunity driven, economy backed way to understand what's real.

**Key Differentiator:**

Most fact checking today jappens behind closed doors, with users having to trust an organization's judgement. This system flips that model by letting truth emerge from the community itself through stake backed voting. Verifiers are rewarded for being accurate and penalized for being careless, creating honest incentivites baked directly into the protocol. Everything from votes to final truth flag is recorded on-chain, making the process transparent and fair. This creates a more open and trustworthy alternative to traditional fact checking, built for the challenges of the current AI era.

## Target User Profile

- **Everyday News Consumers:** They are tired of conflicting claims, AI generated fakes and politicized fact checkers. They want a reliable and unbiased source of truth.

- **Independent Reporters:** Publishes news independently without institutional backing. Values credibility, immutable proof of reporting.

- **Fact Checkers:** Actively evaluates claims, debunks misinformation. Values stake backed incentives, clear rewards for accuracy and tamperproof audit trail.

## User Story ID: ZND-001

**1. User Persona**

**Name:** Aditi

**Role:** Everyday NEws Consumer

**Goal:** Quickly and confidently understand if a news is real or misleading

**2. User Story**

As an everyday news consumer, I want to check the truth status of any news item so that I can trust what I am reading in an age filled with misinformation

**3. User Journey:**

Aditi lands on the homepage of the news verification app.
She sees a clean search bar and a feed of trending verified news.
She types in a headline she saw earlier on social media.
Instatly the app fetches the on-chain record for that news item.

She clicks on the item and is presented with:
- A big truth flag(True/False/Unresolved)
- A timestamp of when it was verified.
- A transparent breakdown of votes cast by stake verified fact checkers.

Feeling confident about the result she leaves the app with clarity, Knowing she finally has a trustworthy, unbiased source of truth.

**4. Acceptance Criteria:**

**Functionality:**

- User can search and open any on-chain news item
- System displays truth flag, vote breakdown, verification timeline

**User Interaction:**

- User lands on homepage and can search immediately without connecting wallet.
- User can view news and expand verifier details.

**Security:**

- The truth status must be sourced directly from on-chain data.
- User cannot influence verification result.


## User Story ID: ZND-002

**1.User Persona**

**Name:** jack

**Role:** Independent Reporter

**Goal:** Publish news with with immutable, verifiable proof of authenticity

**2. User Story**

As an independent repoter, I want to publish news on-chain so that my provably authentic and publicly verifiable/

**3. User Journey**

Jack lands on the platform and clicks on "Connect Wallet"
Once connected, the interface recognizes his address and opens the "Publish News" dashboard.

He writes his headline, pastes his article and attaches couple of supporting documents if available.
He reviews the submission preview, making sure everything is accurate.

He clicks on "Publish".
A wallet transaction pops up, asking for signature.
He signs it, Commiting the entire report to the blockchain.

A success message appears:
"News Successfully Published"

Jack can now track verifier activity:
Votes coming in, stake distribution, consensus movement and dispute phase status.

As verification progresses, he gains credibility through transparent on-chain evidence.

**4. Acceptance Criteria**

**Functionality**

- Reporter can publish news items on-chain
- Reporter can attach evidence
- Platform triggers a verification window.

**User Interaction**

- Reporter connects wallet to access submission dashboard
- Reporter fills details, submits  and signs a transaction
- Reporter sees confirmation and verifcation progress updates

**Security**

- Only wallet owner can publish under their identity
- All submissions stored immutably on-chain
- Reporter can not manipulate verification outcome.


## User Story ID: ZDN-003

**1. User Persona**

**Name:** Maya

**Role:** Fact Checker/Verifier

**Goal:** Evaluate news transparently and earn rewards for accurate verification

**2. User Story**

As a fact checker, I want to review news items and casr stake backed votes so that I can help determine truth and earn rewards for accuracy.

**3. User Journey**

Maya visits the platform and clicks "Connect Wallet"
After connecting, she goes to the "Become a Verifier" section

She reads that verifiers must deposit SOL to join the network.
This initial SOL deposit mints her "Verification Power Token", 
a non transferable token representing her voting power.

She clicks "Stake SOL & Join Network"
Here wallet pops up asking for signature.
She confirms the transaction.

Seconds later, the dashboard updates:
"Congratulation! You are  now a verifier. Your power: 50 VPT"

**Her Verification Workflow**

Maya navigates to the verification dashboard, where all open news items are listed:

- Verification window time remaining
- Supporting evidence for the news, if available

She selects a news item.
She reads the headline, reads the attached document, checks reference and does here independent research.

Feeling confident in her assessment, she clicks on "Vote True".
This time, she does not stake SOL per vote, the vote simply users her existing VPT.

Her wallet pops up for a simple signature confirming here vote.

Maya opens her profile page where she sees:

- Verrification Power
- Rewards earned for accurate votes
- Slashing penalty history(If she ever voted incorrectly)
- Reputation Points

Later when the news item reaches consensus, she receives a notification:
"You voted correctly. Reward distibuted, +0.8 SOL".

Her VPT remains intact unless she voted incorrectly.
Her Stake and reputation grows overtime, allowing her to strengthen her influence by staking more SOL.\

**4. Acceptance Criteria**

**Functionality**

- Verifiers can stake SOL once to join the network.
- Staked SOL mints Verification Power Tokens proportional to stake.
- VPT determines voting power
- Verifiers can stake more sol to increase their VPT
- Reward/Slashing are computed automatically per final verdict.

**User Interaction**

- User connects wallet and stakes SOL to become verifier
- User receives VPT immediately atfer staking
- Verifier browses open news items and votes using their VPT
- Verifier signs a simple vote transaction
- Verifier can track voting history, rewards, VPT balance and penalties

**Security**

- Only wallets with VPT can vote
- VPT must be non-transferable and tied to the wallet identity
- Each verifier can cast only one vote per news item
