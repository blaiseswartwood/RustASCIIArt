CSSE497 Standards Assignment (with XP)

## Outcomes

#### a. Identified stakeholder concerns are addressed by the architecture

#### b. Architecture viewpoints are developed

#### c. Context, boundaries, and external interfaces of the system are defined

#### d. Architecture views and models of the system are developed

#### e. Concepts, properties, characterisics, behaviors, functions, or constraints that are significant to architecture decisions of the system are allocated to architectual entities.

#### f. System elements and their interfaces  are identified.

#### g. Architecture candidates are assesed.

#### h. An architectural basis for processes throughout the life cycle is acheived.

#### i. Alignment of the architecture with requirements and design characterisics is achieved.

#### j. Any enabling systems or services needed for architecture defintion are available.

#### k. Traceability of architecture elements to stakeholder & system/software requirments is developed.

# 6.4.2 Stakeholder Needs and Requirements Definition process (using outcomes letters associated with this section)

## 1. Prepare for Stakeholder Needs and Requirements Definition

### a. Identify the stakeholders who have an interest in the software system throughout its lifecycle.

- **Outcome Addressed**: a
- **Extreme Programming Implementation**: Identify and actively get involved with stakeholders in the system. “One is that no matter what the client says the problem is, it is always a people problem. Technical fixes alone are not enough” (Beck and Andres 26).
- **Project Implementation Example**: Our project has stakeholders of ASCII Art animation Rust programmers and their potential clients who would use it (such website users). Our client is one of these programmers.

### b. Define the stakeholder needs and requirements definition strategy

- **Outcome Addressed**: d
- **Extreme Programming Implementation**: Use stories to capture stakeholder needs and requirements, which will change over time. "Make sure what you are doing has business value, meets business goals, and serves business needs.” (Beck and Andres 21).
- **Project Implementation Example**: Our project uses stories in GitHub Issues to keep a record of our clients changing needs and features.

### c. Identify and plan for the necessary enabling systems or services needed to support stakeholder needs and requirement definitions

- **Outcome Addressed**: h
- **Extreme Programming Implementation**: Use incremental design to constantly update and enable our services to for stakeholder needs. “Generally, the closer customer needs and development capabilities are, the more valuable development becomes.” (Beck and Andres 34)
- **Project Implementation Example**: We are changing our system design to incorporate async operations to render ASCII Art more efficiently as requested by the client.

### d. Obtain or aquire access to enabling systems of services to be used.

- **Outcome Addressed**: h
- **Extreme Programming Implementation**: Using continuous integration ensures systems and services are obtained and available to use. “Its ability to flexibly schedule the implementation of functionality, responding to changing business needs.” (Beck and Andres 13)
- **Project Implementation Example**: Our project uses continuous integration to ensure and test that all of the other libraries we rely on such as glium can be run non-locally.

## 2. Define stakeholder needs.

### a. Define context of use within the concept of operations and the prelimiary life cycle concepts

- **Outcome Addressed**: b
- **Extreme Programming Implementation**: Use incremental design to account for operations and prelimiary life cycles to constantly update these operations for stakeholders. “Incremental design is a way to deliver functionality early and continue delivering functionality weekly for the life of the project.” (Beck and Andres 21)
- **Project Implementation Example**: We use incremental design to make constant adjustments a client wants to our ASCII art - for example, we brainstormed a singleton pattern for our event queue but quickly disposed of it after hearing client feedback.

### b. Identify stakeholder needs

- **Outcome Addressed**: d
- **Extreme Programming Implementation**: Identify stakeholder needs through user stories and constantly interacting with them. “XP is asking to get some of your human needs met through software development.” (Beck and Andres 15)
- **Project Implementation Example**: We identify our client's needs by having a list of stories each week and adding any ideas the client has to this list of stories to work on (such as wanting async operations).

### c. Prioritize and down-select needs

- **Outcome Addressed**: e
- **Extreme Programming Implementation**: Through the weekly cycle have the client pick stories. “Have the customers pick a week’s worth of stories to implement this week.” (Beck and Andres 28)
- **Project Implementation Example**: We identify our client's needs by having a list of stories each week and allowing him to choose the most important ones (such as async/await over a new animation type).

### d. Define the stakeholder needs and rationale

- **Outcome Addressed**: d
- **Extreme Programming Implementation**: Instead of defining the stakeholder needs and rationale, by constantly interacting with them through a weekly cycle, have the client articulate their needs. “The point of customer involvement is to reduce wasted effort by putting the people with the needs in direct contact with the people who can fill those needs.” (Beck and Andres 28)
- **Project Implementation Example**: By allowing our client to pick weekly stories, it is clear the client prioritizes fast efficient operations (using glium for GPU and wanting async operations) for ASCII art.

## 3. Develop the operational concept and other life cycle concepts

### a. Define a representative set of scenarios to identify the required capablities that correspond to anticipated operation and other life cycle concepts.

- **Outcome Addressed**: b
- **Extreme Programming Implementation**: Rather than speculate and make user scenarios, ask the client to get these scenarios on a weekly basis. Directly implement and test these concepts. “Make sure what you are doing has business value, meets business goals, and serves business needs." (Beck and Andres 28)
- **Project Implementation Example**: Our client gives us a story, such as to implement a fading trail animation, and we directly code and test that in our system.

### b. Identify the factors affecting interactions between users and the systems

- **Outcome Addressed**: b
- **Extreme Programming Implementation**: Identify these factors through repeated interactions with the client directly, rather than doing it all at the beginning. “Planning is a form of necessary waste. It doesn’t create much value all by itself. Work on gradually reducing the percentage of time you spend planning.” (Beck and Andres 28)

#### i. Anticipated physical, mental, and learned capabilities of the users

- **Project Implementation Example**: We directly show our client our system and adjust our implementation to make it easier to use. For example, using async/await in a for loop with a wait time is easier than wrapping several closures in each other one after the other.

#### ii. Workplace, environment, and facilities, including other equipment in the context of use

- **Project Implementation Example**: We directly ask when our ASCII art animation system would be used - typically in a non-stressful setting simply for entertainment.

#### iii. Normal, unusual, and emergency conditions

- **Project Implementation Example**: Under normal circumstances, our system will draw art normally. Under heavy testing systems, we are building stress tests to test how many animations can be on the screen for the unsual conditions.

#### iv. Operator and user recruitment, training and culture

- **Project Implementation Example**: We will provide some basic coding examples of each type of animation once we have completed the system more thoroughly.

## 4. Transform stakeholder needs into stakeholder requirements

### a. Identify the constraints on a system solution

- **Outcome Addressed**: c
- **Extreme Programming Implementation**: Constraints show up in a system solution when there is more and more work (or work arounds) revolving around that code. “How do you find the constraint in a system? Work piles
   up in front of a constraint. There are not piles of dried clothes sitting waiting to be folded; there are piles of wet clothes sitting waiting to be dried.” (Beck and Andres 42)
- **Project Implementation Example**: Our client found a constraint in our system by looking at how much work it takes to chain animations by wrapping multiple closures over and over again. Thus, our client wants us to alleviate this constraint using async/await.

### b. Identify the stakeholder requirements and functions that relate to critical quality characteristics, such as assurance, safety, security, environment, or health

- **Outcome Addressed**: f
- **Extreme Programming Implementation**: Rather than speculating and making a list of these characteristics that may change, directly ask the clients about them as necessary to develop and do everything to achieve these qualities. “Each increase in quality leads to improvements in other desirable project properties, like productivity and effectiveness, as well. There is no apparent limit to the benefits of quality, only limits in our ability to understand how to achieve higher quality.” (Beck and Andres 24)
- **Project Implementation Example**: We have not had any of these critical quality characteristics come up yet in our system design for ASCII art. The closest would be speed/efficiency to draw lots of complex art.

### c. Define stakeholder requirements, consistent with life cycle concepts, scenarios, interactions, constraints, and critical quality characteristics

- **Outcome Addressed**: e
- **Extreme Programming Implementation**: Use the quarterly cycle to integrate these different concepts and what is being built to the overall stakeholder vision. “The separation of “themes” from “stories” is intended to address the tendency of the team to get focused and excited about the details of what they are doing without reflecting on how this week’s stories fit into the bigger picture.” (Beck and Andres 29)
- **Project Implementation Example**: We do not have a quarterly cycle (as discussed in class), so instead in our user stories we ensure all the stories address the main ASCII art requirement of speed and efficiency for complex animations.

## 5. Analyze stakeholder requirements

### a. Analyze the complete set of stakeholder requirements

- **Outcome Addressed**: e
- **Extreme Programming Implementation**: Use the quarterly cycle to pick the main stakeholder themes. “Generally, the closer customer needs and development capabilities are, the more valuable development becomes.” (Beck and Andres 29)
- **Project Implementation Example**: As stated earlier, we do not have a quarterly cycle. Instead, we look at the requirements of our client through the stories he picks, which focus on fast, efficient ASCII art animation.

### b. Define critical performance measures that enable the assessment of technical achievement

- **Outcome Addressed**: f
- **Extreme Programming Implementation**: We discuss these requirements through our user stories but only focus on a few essential ones. If its not absolutely imperative, we ignore it. “Out of one thousand pages of “requirements”, if you deploy a system with the right 20% or 10% or even 5%, you will likely realize all of the business benefit envisioned for the whole system.” (Beck and Andres 28)
- **Project Implementation Example**: We asked our client the most important factor, which we found to be speed/efficiency for drawing advanced ASCII art animations.

### c. Feed back the analyzed requirements to applicable stakeholders to validate that their needs and expectations have been adequately captured and expressed

- **Outcome Addressed**: g
- **Extreme Programming Implementation**: In weekly cycles, get client feedback directly from the previous set of stories implemented. “Have the customers pick a week’s worth of stories to implement this week” (Beck and Andres 28)
- **Project Implementation Example**: We always show our client our most recent build of animations and report our progress and troubles such as implementing the executor component of async.

### d. Resolve stakeholder requirements issues

- **Outcome Addressed**: g
- **Extreme Programming Implementation**: Resolve stakeholder requirement issues through user stories at every weekly meeting and focus on resolving larger ones at quarterly cycles through conversation, not documentation. “The clearer the quarterly cycle becomes at expressing the business priorities, the slimmer the requirement document needs to be.”(Beck and Andres 28)
- **Project Implementation Example**: We usually recieve positive feedback that we have made good progress weekly with our user stories (and thus are solving the client's issues), but have at times gotten feedback that we did not do a story correctly
   (for generating different text fonts, we were going to hardcode a texture atlas, whereas the client would wanted a library that would generate only the specific font on the spot)

## 6. Manage the stakeholder needs and requirements definition

### a. Obtain explicit agreement with designated stakeholders on the stakeholder requirements

- **Outcome Addressed**: g
- **Extreme Programming Implementation**: Directly have our clients involved in weekly planning so they can choose the requirements to work on. “Visionary customers can be part of quarterly and weekly planning.” (Beck and Andres 34)
- **Project Implementation Example**: We always ensure that our client picks our stories - we never choose the stories without our client permission.

### b. Maintain traceability of stakeholder needs and requirements

- **Outcome Addressed**: i
- **Extreme Programming Implementation**: Stakeholder needs and requirements are only maintained briefly through user stories and then live in the code. “Maintain only the code and the tests as permanent artifacts. Generate other documents from the code and tests. Rely on social mechanisms to keep alive important history of the project” (Beck and Andres 35)
- **Project Implementation Example**: We have our GitHub Issues with the user stories which are soon closed as soon as they are completed.

### c. Provide key artifacts and information items that have been selected for baselines

- **Outcome Addressed**: i
- **Extreme Programming Implementation**: We only provide the code and tests for permanent artifacts. The rest are generated on the fly and then diposed of. “Any artifacts contributing to these two sources of value (code and tests) are themselves valuable. Everything else is waste” (Beck and Andres, pg. 35)
- **Project Implementation Example**: Our client has not asked for any artifacts other than the code, which we gave the client access to.

# 6.4.4 Architecture Definition Process

## 1. Prepare for architecture definition

### a. Review pertinent information and identify key drivers of the architecture
- **Outcome Addressed**: i
- **Extreme Programming Implementation**: "Make sure what you are doing has business value, meets business goals, and serves business needs. For example, solving the highest priority business need first maximizes the value of the project." (Beck and Andres) You have to know what your clients needs are in order to fill them, or the software you cretae might not actually be of use to our client.
- **Project Implementation Example**: Since this project isn't for a organization, business, or to be a product, we weren't overly concerned with competitor products, market studies, or organization polices and directives.We want to ensure to design that our "screen" which displays things on the screen using our generated vertex buffer and shaders, was flexible enough to be replaced at a later date. This is important, because it ensures that the project is flexible enough to be used in different applications, which is what our client wants. Since this project is meant to be written in Rust, we want to structure it in a Rust-like way, fully utlizing the properties of the language. We also want the operations to be fast, since this is an animation software, so we're designing while keeping an eye on the runtime, limitations of the system, and off=loading responsbility from the CPU to GPU when possibile.

### b. Identify stakeholder concerns
- **Outcome Addressed**:  a
- **Extreme Programming Implementation**: "Make people whose lives and business are affected by your system part of the team. Visionary customers can be part of quarterly and weekly planning. They can have a budget, a percentage of the available development capacity, to do with as they please." (Beck and Andres, pg. 34)
- **Project Implementation Example**: Our client was concerned about support over the life-cycle of the software and the language updating and causing errors with outdated packages, since our client, Dr. Buffalo, said that is part of what killed the previous version of this project. So, we limit the number of open source packages we use, and pay attention to making sure we keep track of what verisions of Rust and open source packages we use. Our client was also concerned about evolution of the software system and it's ability to be adapted to several different outputs, as mentioned in the previous task.

### c. Define the Architecture Definition roadmap, approach, and strategy
- **Outcome Addressed**:  h, g
- **Extreme Programming Implementation**: "Invest in the design of the system every day. Strive to make the design of the system an excellent fit for the needs of the system that day. When your understanding of the best possible design leaps forward, work gradually but persistently to bring the design back into alignment with your understanding." (Beck and Andres, pg. 30)
- **Project Implementation Example**: Our only stakeholder is really our client, as it is largely for his personal use. Following XP guidelines, we meet with our client weekly. This gives us many opportunies to communicate with our stakeholder. Our current measurement process is defined by our stress-tests confirming how many vertexes/events/animations at a time that our system can handle, which is communicated to our client. We also do some testing so we can identitify the largest chokeholds and think about how we might make them faster. Again, following XP principles, we dedicate time invested in our architecture design & review every time we code. 

### d. Define architecture evaluation criteria	based on stakeholder concerns and key requirements
- **Outcome Addressed**: i
- **Extreme Programming Implementation**: "Generally, the closer customer needs and development capabilities are, the more valuable development becomes." (Beck and Andres, pg. 34) The architecture must align and allow for the customer needs to be met.
- **Project Implementation Example**:  How fast the system runs is both a stakeholder concern and key requirement of the system, as our client wants to create large scale animations. As mentioned in our Stategy above, we perform stress test to evaluate how well the architecture achieves its goals. We also evaluate how difficult it is to create animations from a user perspective (a key requirement of the project) and communicate that to our client, as well as potential stories to make it easier / asking him what behavior he would prefer from a user perspective. Overall, our client has given us a few general benchmarks, but we have had to define our own evaluation criteria based on stakeholder concerns which we then communicate to the client for most of the system. 

### e. Identify and plan for the necessary enabling systems (5.2.3) or services needed to support the Architecture Definition process
- **Outcome Addressed**: j
- **Extreme Programming Implementation**: "Find a starting place, get started, and improve from there." (Beck and Andres, 22). XP's focus isn't really on the exact steps to start development, just that you just start develop as quickly as possible, constantly designing and constanntly readjusting along the way.
- **Project Implementation Example**: We have set up a pipeline for building, testing, and integration using RustC, Clippy, GitHub Code Coverage/Continuous Integration as our enabling systems. We also rely on Cargo as our package manager and import Glium as our graphics interface.

### f. Obtain or acquire access to the enabling systems (5.2.3) or services to be used
- **Outcome Addressed**: j
- **Extreme Programming Implementation**: N/A. See task e above.
- **Project Implementation Example**: All of our enabling systems are open source, and were easily integrated through Github (code coverage, continuous integration), Rust (linter, cargo), and cargo (all libraries). 



## 2. Develop architecture viewpoints

### a. Select, adapt, or develop viewpoints and model kinds based on stakeholder concerns
- **Outcome Addressed**: b
- **Extreme Programming Implementation**: "Plan using units of customer-visible functionality" (Beck and Andres, pg. 28). Although the stories aren't directly to architecture viewpoints, those conversations are often the ones where we get to see the most important viewpoints to our client, and become aware of ones that need to be adapted or developed.
- **Project Implementation Example**: As previously mentioned, we meet with our client on a weekly basis, which helps us keep updated on stakeholder concerns, and whether we need to adapt or develop new viewpoints. We bring up stories, and talk about what value they would give the project, and how they will relate to the viewpoints of the system. There are a few main categories of concerns that we keep track of: the system's ease of use and flexibility for users, performance and capabilities of the software, and lifecycle and adaptability. Each of these viewpoints is applicable to particular parts of the system and architecture design that we can then select model kinds. 

### b. Establish or identify potential architecture framework(s) to be used in developing models and views
- **Outcome Addressed**: g
- **Extreme Programming Implementation**: "Tests can communicate architectural intent." (Beck and Andres, pg. 38). Since we use test-first programming, when developing tests we use what we know of the project's views, models, and framework to ensure that we match all of those principles when writing our code.
- **Project Implementation Example**: In general XP discourages architectural artifacts, so the architecture frameworks are only our guiding principles in discussion of the design and implementation. Test-Driven development somewhat matches the definition of an architecture framework in that it stipulates development of the architecture follows the development of tests which model our system, however beyond this we try to follow principles in our design which promote good encapsulation and a clean interface for use of our library. 

### c. Capture rationale for selection of framework(s), viewpoints and model kinds
- **Outcome Addressed**: a, g, d
- **Extreme Programming Implementation**: "Product managers encourage communication between customers and programmers, making sure the most important customer concerns are heard and acted on by the team." (Beck, Pg. 39). Although this isn't directly realted to framework(s), viewpoints and model kinds, since our customer doesn't have direct access to all of it, we do practice Real Customer Involvement, which is how we get a lot of our rationale.
- **Project Implementation Example**: Our rationale for selecting framework, viewpoints and model kinds is based on our stakeholder's concerns, and adherence to test-driven development. We based our principles of interface design on the desires of our client as expressed through user stories. For example, our interface is designed to use Async/Await patterns as explicitly requested by the client.

### d. Select or develop supporting modelling techniques and tools.
- **Outcome Addressed**: d
- **Extreme Programming Implementation**: "It’s easy to get carried away programming and put in code “just in case.” By stating explicitly and objectively what the program is supposed to do, you give yourself a focus for your coding. If you really want to put that other code in, write another test after you’ve made this one work." (Beck and Andres, 30)
- **Project Implementation Example**: Our modelling system is based on test-driven development. We build test cases which provide intended mappings of inputs to outputs for our program, and these tests serve as our model for the how the system should behave. Toward this, we use Rust's integrated testing framework which allows us to easily create these tests in place, and use them to validate our code both during development and for regression testing. We are somewhat limited in what can be modelled with tests, since much of our project is visual, so in these cases we use client descriptions of visuals as our model for how the system should behave, and check in with our client to confirm that our interpretation of this model is accurate as we develop the system. In general, since XP discourages the creation of software artifacts not in the tests/code, we also keep track of these kinds through conversations within the team and with the client.



## 3. Develop models and views of candidate architectures

### a. Define the software system context and boundaries in terms of interfaces and interactions with external entities
- **Outcome Addressed**: e
- **Extreme Programming Implementation**: "The objection I hear to customer involvement is that someone will get exactly the system he wants, but the system won’t be suitable for anyone else." (Beck and Andres, 34). If this is the case, then we're doing it right, since our project has only one real stakeholder.
- **Project Implementation Example**: Again, our project is less focused on larger business and mission analysis since this project is for a single client's personal use. However, we have identified how our client would like to use the project, and possible further uses he might want to see it developed into. For example, he may want to have our animation system inputted into a terminal so you could have cool hacker-type animations in response to running tests and the like. 

### b. Identify architectural entities and relationships between entities that address key stakeholder concerns and critical software system requirements
- **Outcome Addressed**: a
- **Extreme Programming Implementation**: "Some of our customers are great. They write good stories. They write acceptance test criteria. They help testers write acceptance tests." (Beck and Andres, 53)
- **Project Implementation Example**: Because our client has largely left the implementation of the program up to us, with only the broad strokes and a few use cases, we have very few requirements that are not key to stakeholder concerns. But the concerns we have are fairly exacting. So far, the requirments for the program have changed only slightly. Largely, when we ask if he has a preference for how exactly the user should use the program or a particular animation, he indicates that he does not care, and wants us to do the decision-making. Given this, we have a very defined list of critical requirements and concerns. Because of the size of this project, identifying architectural entities and relationships between entities that address these concerns has not been difficult at all. 

### c. Allocate concepts, properties, characteristics, behaviors,  functions, or constraints that are significant to architecture decisions of the software system to architectural entities
- **Outcome Addressed**: e
- **Extreme Programming Implementation**: "Tests can communicate architectural intent." (Beck and Andres, pg. 38)
- **Project Implementation Example**: We are very aware of which features in our program correspond to each architectural decisions, the logic of which is often captured in our story descriptions. 

### d. Select, adapt, or develop models of the candidate architectures of the software system
- **Outcome Addressed**: d
- **Extreme Programming Implementation**: "... including customers in the development process fosters trust and encourages continued improvement." (Beck and Andres, pg. 34)
- **Project Implementation Example**:  Because our client is very well-versed in programming, he often requests to hear how we plan to implement a story architecture-wise, and often has suggestions, which we, of course, factor into our decisions when developing and selectin candidate architectures. 

### e. Compose  views  from  the  models  in  accordance  with  identified  viewpoints  to  express  how  the architecture addresses stakeholder concerns and meets stakeholder and system/software requirements
- **Outcome Addressed**: i
- **Extreme Programming Implementation**: "Its [Xp's] reliance on oral communication, tests, and source code to communicate system structure and intent." (Beck and Andres, pg. 13) 
- **Project Implementation Example**: We use views based on our identified viewpoints to drive our test creation. We converse with our client to distill the viewpoints into stories, which contain their view and motivations, then we use this to develop our tests which serve as our model. XP dissuades non-code/test artifacts, so these views remain conceptual as a part of discussion while developing tests.

### f. Harmonize the architecture models and views with each other
- **Outcome Addressed**: d
- **Extreme Programming Implementation**: "What matters most in team software development is communication. When problems arise in development, most often someone already knows the solution; but that knowledge doesn’t get through to someone with the power to make the change." (Beck and Andres, pg. 19)
- **Project Implementation Example**: Toward unifying the architectural model with the views, we constantly re-evaluate the efficacy of our testcases (our models) and their relationship with the stories. We also use code coverage to ensure our testcases are comprehensively modeling the functionality of the architecture. On a interpersonal level, we ensure that architecture models and views align with each other by pair programming and checking each other's work and constant reaffirming with each other and the client what exactly our architecture models, views, and viewpoints are. 



## 4. Relate the architecture to design

### a. Identify software system elements that relate to architectural entities and the nature of these relationships.
- **Outcome Addressed**: f, a
- **Extreme Programming Implementation**: "What matters most in team software development is communication. When problems arise in development, most often someone already knows the solution; but that knowledge doesn’t get through to someone with the power to make the change." (Beck and Andres, pg. 19)
- **Project Implementation Example**: In general XP speaks against artifacts relating to architecture, so our architectural entities are defined only by our teams communication. Toward building these elements we utilize Rust's module system to separate our software system into self contained software elements which match with our discussed architectural entities. Due to the lack of architecture artifacts, the relationship between these is only as strong as our discussion of the system, and after the system elements are designed, they serve as our architectural entities as the code is our only artifact of architecture.


### b. Define the interfaces and interactions among the software system elements and external entities.
- **Outcome Addressed**: c
- **Extreme Programming Implementation**: N/A. XP doesn't speak about interfacing with external systems. An argument could be made that test driven development is XP's solution to defining an interface - we should design tests to simulate the use of the interface. However, this doesn't approach the actual interface design, just how to design the code behind it.
- **Project Implementation Example**: In our case, the system is self-contained and acts as a library. There are no networking or integration features beyond the exposed API we provide for using the library. As such, our software elements are the interface, since we can control access to these elements using rust's publicity features. We define these interactions based on discussion with our client and our stories (what user-facing features should be available). 


### c. Partition, align and allocate requirements to architectural entities and system elements.
- **Outcome Addressed**: e
- **Extreme Programming Implementation**: "I think of design as a system of beneficially related elements. Each word in this definition is loaded with meaning. 'Elements' suggests that systems can’t be comprehended only as wholes." (Beck and Andres, pg. 47). XP asserts that design of systems must be inherently modular, and that the only reasonable way to approach a system is to delegate functionality to subsections.
- **Project Implementation Example**: Since our code is our architecture artifact, this allocation is the same for system elements and architectural entities. We use the stories to guide the outward facing interface, and discuss how our internal architecture can achieve the goals of that interface. We use this discussion to guide our separation of architectural entities and how they interact with each other.

### d. Map software system elements and architectural entities to design characteristics
- **Outcome Addressed**: k
- **Extreme Programming Implementation**: "Stories.", "Plan using units of customer-visible functionality.", "Software development has been steered wrong by the word “requirement”, defined in the dictionary as “something mandatory or obligatory.” The word carries a connotation of absolutism and permanence, inhibitors to embracing change. " (Beck and Andres, pg. 28).
- **Project Implementation Example**: We design our system such that the software system elements map directly to the distinct features which our stories (on GitHub) describe. We use the rust module system to ensure that distinct features have distinct architectural entities/software system elements.

### e. Define principles for the software system design and evolution.
- **Outcome Addressed**: h
- **Extreme Programming Implementation**: "The specification of the project is continuously refined during development, so learning by the customer and the team can be reflected in the software." (Beck and Andres, pg. 14)
- **Project Implementation Example**: Our design process is guided by encapsulation and reduced coupling. This allows us to easily extend or modify existing software system elements without causing regression. In general this means appropriately separating our work into Rust modules. More generally, we use XP's continous feedback development to constantly define and refine our software system design with our client over the course of its evolution.


## 5. Assess architecture candidates.

### a. Assess each candidate architecture against constraints and requirements.
- **Outcome Addressed**: g
- **Extreme Programming Implementation**: "Pair programming is a dialog between two people simultaneously programming (and analyzing and designing and testing) and trying to program better." (Beck and Andres, pg. 27)
- **Project Implementation Example**: For us, this process takes place during discussion and pair programming. There is no artifact of the architecture beyond the code, so we discuss potential implementations during pair programming and compare their advantages and disadvantages. Part of this comparison is assessing how effectively the design will meet our requirements, and if we find this to be insufficient, we will discuss other options before we implement the system element.

### b. Assess each candidate architecture against stakeholder concerns using evaluation criteria.
- **Outcome Addressed**: g, a
- **Extreme Programming Implementation**: "Pair programmers: Keep each other on task. Brainstorm refinements to the system. Clarify Ideas." (Beck and Andres, pg. 27)
- **Project Implementation Example**: While pair programming we ensure that our design effectively meets the needs of the story we are currently working on. This ensures that our architecture effectively addresses the stakeholder concerns which the story is related to. Essentially we defer this process such that we ensure the stories match the stakeholder concerns, then when we develop we ensure that the development process match the story's requirements. 

### c. Select the preferred architecture(s) and capture key decisions and rationale.
- **Outcome Addressed**: i
- **Extreme Programming Implementation**: "XP teams work hard to create conditions under which the cost of changing the software doesn’t rise catastrophically. The automated tests, the continual practice of improving the design, and the explicit social process all contribute to keep the cost of changes low." (Beck and Andres, pg. 30)
- **Project Implementation Example**: We decide on our architecture through discussion while pair programming. XP discourages rational artifacts, but when necessary we add comments to the related modules explaining the rational behind the design. If it is a large decision, we discuss as a group and perhaps our client, depending on the situation. Between us, we have all the requisite knowledge of the views, models, and general architecture we need to keep in mind. Although we generally take, small safe steps, we still might need to decide a direction to start in. In general this rational should be obvious through the exposed interface, though when this meaning is obscured, we use in-code documentation (such as tests) to capture it. 

### d. Establish the architecture baseline of the selected architecture.
- **Outcome Addressed**: h
- **Extreme Programming Implementation**: "Maintain only the code and the tests as permanent artifacts. Generate other documents from the code and tests. Rely on social mechanisms to keep alive important history of the project." (Beck and Andres, pg. 35)
- **Project Implementation Example**: Since XP promotes artifact-less design, our architecture baseline is minimal - We represent views in the stories, and model using tests as dictated by test-driven development. Beyond this, the only part of the selected architecture which is required to establish a baseline is our design of the architectural entities.

## 6.Manage the selected architecture

### a. Formalize the architecture governance approach and specify governance‐related roles and responsibilities, accountabilities, and authorities related to design, quality, security, and safety
- **Outcome Addressed**: i
- **Extreme Programming Implementation**: "Dictating practices to a team destroys trust and creates resentment. Executives can encourage team responsibility and accountability." " The price of the improvement is closer collaboration and engagement with the whole team." (Beck and Andres, pg. 32). Because we don't really have assigned roles, we've all adopted responsibility for maintaining respect to the architecture. So, we generally make decisions with far-reaching impacts as a team, so that we all will have closer engagement with the project and share the responsbility between all of us.
- **Project Implementation Example**: For our project, we generally share responsibility for managing architecture. This is because our team is small enough that it's feasible. We delegate design responsibily to pairs, and occassionally swap which pair is working. In this way we all share responsibility and accountability. When larges changes to the architecture are required or we need to decide in which direction to make a small step in, we discuss as a group before any individual or pair makes a significant change.

### b. Obtain explicit acceptance of the architecture by stakeholders
- **Outcome Addressed**: a
- **Extreme Programming Implementation**: The closest XP comes to mentioning this is: "Have the customers pick a week’s worth of stories to implement this week." In general, XP doesn't speak about letting the clients see the architecture and promotes client interaction in terms of requirements and stories - the actual things the clients wants ignoring the technical details. Our situation is somewhat different in that our client understands the technical details, but XP leaves room for how exactly to manage these interactions.
- **Project Implementation Example**: We obtain stakeholder acceptance through approval and selection of stories and through the weekly demos/feedback cycle. This allows us to get feedback about the functionality of the architecture while allowing us to abstract somewhat for the client. In our particular case, we allow our client to see more of the internals than other teams might since our client is well in touch with how the system should work internally. 

### c. Maintain concordance and completeness of the architectural entities and their architectural characteristics
- **Outcome Addressed**: e, i
- **Extreme Programming Implementation**: "What matters most in team software development is communication. When problems arise in development, most often someone already knows the solution; but that knowledge doesn’t get through to someone with the power to make the change." (Beck and Andres, pg. 19)
- **Project Implementation Example**: As we revisit sections of our code, we make sure to maintain our previous architectural frameworks, maintaining modularity and scalability as much as possible. This is helped by the fact that we practice pair programming and conversations between us, which helps us keep the architectural complete and concordant with our architectural characterestitics, since we are all seeking to maintain adherence to the architectural characteristics. When we need to make a change to our architecture we discuss to make sure our changes uphold these standards. We try to avoid any change which might cause issues in the forseeable future and opt to instead consider alternatives when potential issues are recognizable.

### d. Organize, assess and control evolution of the architecture models and views to help ensure that the architectural intent is met and the architectural vision and key concepts are correctly implemented
- **Outcome Addressed**: h, e
- **Extreme Programming Implementation**: "The specification of the project is continuously refined during development, so learning by the customer and the team can be reflected in the software." (Beck and Andres, pg. 14)
- **Project Implementation Example**: While developing, we are constantly evaluating how effective the code is at meeting the requirements and with more solidified architecture, we can evaluate the feasibility of requirements more accurately. As we find feasibility changes in the requirements we may reassess those requirements and update stories with the client, update test cases (the model) to better reflect the changes in architecture or re-evaluate our architecture. This allows our architecture to remain congruent with our models and views.

### e. Maintain the architecture definition and evaluation strategy
- **Outcome Addressed**: h
- **Extreme Programming Implementation**: The most relevant discussion to this may be in the role of the Interaction Designers: "Interaction designers on an XP team choose overall metaphors for the system, write stories, and evaluate usage of the deployed system to find opportunities for new stories." While this only tangentally touches this task, it does imply that there should be continuous evaluation of the system and it's architecture. It's naturally extensible that the developers would evaluate new information as the architecture develops and work with the client to update the stories accordingly and better match real technical/feasibility limitations.
- **Project Implementation Example**: As we work on the software and find limitations or opportunities, we adjust the architecture accordingly. We ensure that the architecture meets the requirements adequately and that our stories and requirements match the limitations of what the system can do/be extended to do. When changes are neccessary we ensure that we maintain our existing architectural frameworks. 

### f. Maintain traceability of the architecture
- **Outcome Addressed**: k
- **Extreme Programming Implementation**: XP doesn't really address this directly, but it does advocate for documenting with tests, so: "Maintain only the code and the tests as permanent artifacts. Generate other documents from the code and tests. Rely on social mechanisms to keep alive important history of the project." (Beck and Andres, pg. 35)
- **Project Implementation Example**: By maintaining that our requirements correspond directly to architectural entitites (in our case rust modules), traceability is maintained between the requirements of the system and it's architecture. When this traceability is less obvious (usually due to a more complex design being required with multiple internal entities), we provide in-code documentation linking software system elements back to their stories (and thus the system requirements and stakeholder concerns which that element addresses). And of course, we can look back at our stories to track the development of features requested by our client.

### g. Provide key artifacts and information items that have been selected for baselines
- **Outcome Addressed**: i, k
- **Extreme Programming Implementation**: "Maintain only the code and the tests as permanent artifacts. Generate other documents from the code and tests. Rely on social mechanisms to keep alive important history of the project." (Beck and Andres, pg. 35)
- **Project Implementation Example**: Since XP maintains only one artifact (the code), providing key artifacts is as simple as providing access to the code. The code includes traceability back to user stories, and relevant documentation when necessary. Additionally, this allows us to roll provision of this information and these artifacts into the weekly demo/feedback cycle.

# 6.4.5 Design Definition Process

## 1. Prepare for Stakeholder Needs and Requirements Definition

### a. Identify the stakeholders who have an interest in the software system throughout its lifecycle.

- **Outcome Addressed**: a
- **Extreme Programming Implementation**:  “XP begins with the assumption that a team of people will be working together toward a common goal.” This continuous feedback between the team and the client is what makes it XP.
- **Project Implementation Example**: In our project, we hold weekly meeting with our client to keep them up to date on our progress and get direct feedback on what the want.

### b. Define the design definition strategy, consistent with the selected life cycle model and anticipated design artifacts.

- **Outcome Addressed**: c
- **Extreme Programming Implementation**: "Invest in the design of the system every day. Strive to make the design of the system an excellent fit for the needs of the system that day." This constant reevaluation of our design and making it fit what we need is that makes it XP.
- **Project Implementation Example**: Everytime we sit down to do a pair-programming session we quickly reevaluate whether our current desin is the optimal way of doing things or if it needs improvemnt.

### c. Select and prioritize design principles and design characteristics.

- **Outcome Addressed**: b
- **Extreme Programming Implementation**: "The advice to XP teams is not to minimize design investment over the short run, but to keep the design investment in proportion to the needs of the system so far." Keeping design to a MVP so it meets the clients current needs with the ability to develop it further is what makes this XP.
- **Project Implementation Example**: We have desined our design to be small and lighweight while still keeping high modularity and scalibility.

### d. Identify and plan for the necessary enabling systems or services needed to support design definition.

- **Outcome Addressed**: g
- **Extreme Programming Implementation**: "Synchronous builds also create positive pressure for a short, clear feedback cycle." Using synchronous integration allows planning and updating of services needed.
- **Project Implementation Example**: Our team used synchronous intergration to help identify what enabling systems would be the most beneficial every time we developed and ensured they were supporting our design.

### e. Obtain or acquire access to the enabling systems or services to be used.

- **Outcome Addressed**: h
- **Extreme Programming Implementation**: "Connecting money flow directly to software development provides accurate, timely information with which to drive improvement."
- **Project Implementation Example**: We are developing our project using Rust, GLSL, and VSC all free and easily accesable systems for people to use.

## 2. Establish designs related to each software system element.

### a. Transform architectural and design characteristics into the design of software system elements.

- **Outcome Addressed**: a
- **Extreme Programming Implementation**: "XP lets you adapt by making frequent, small corrections; moving towards your goal with deployed software at short intervals." This allows for quick and experimental testing to know if the project is going in the right direction.
- **Project Implementation Example**: Our project architecture works off of weekly cycles with quick implementation and an adaptation.

### b. Define and prepare or obtain the necessary design enablers.

- **Outcome Addressed**: f
- **Extreme Programming Implementation**: "Include on the team people with all the skills and perspectives necessary for the project to succeed." having a team of a wide variety of skills and experiences with strong cohesion allows for nessisary ability to success.
- **Project Implementation Example**: Our team consists of people who have some inital experience with rust and glsl and others that have no experience with it. This let us teach eachothe the matieral and lead to us having a better understanding of our design and what we will be implementing.

### c. Examine design alternatives and feasibility of implementation.

- **Outcome Addressed**: e
- **Extreme Programming Implementation**: "The result is systems that can start small and grow as needed without exorbitant cost" Being a small nimble team and systems allows for quick implementation in a better alternative for implementation arrises.
- **Project Implementation Example**: Our team consists of 4 people and we are supposed to finish our project is less than a year. Being a close knit team and open to others ideas allows us to quickly adopt new alternatives and test them out.

### d. Refine or define the interfaces among the software system elements and with external entities.

- **Outcome Addressed**: d
- **Extreme Programming Implementation**: "If the goal is to burn a CD, burn a CD. If the goal is to deploy a web site, deploy a web site, even if it is to a test environment."
- **Project Implementation Example**: Our project involves creating an external library that. Our end product will be interfacable as a rust library that can be download publically through cargo and then immedietly used.

### e. Establish the design artifacts.

- **Outcome Addressed**: g
- **Extreme Programming Implementation**: "Maintain only the code and the tests as permanent artifacts. Generate other documents from the code and tests. Rely on social mechanisms to keep alive important history of the project." This minimalization and only keeping code as documentation is whtat makes this extreme.
- **Project Implementation Example**: The only design artifact we hare are 1 off designs that our clients want. Everything else should be visable through reading our code and understanding how it works

## 3. Assess alternatives for obtaining software system elements

### a. Determine technologies required for each element composing the software system.

- **Outcome Addressed**: c
- **Extreme Programming Implementation**: "XP teams work hard to create conditions under which the cost of changing the software doesn’t rise catastrophically. The automated tests, the continual practice of improving the design, and the explicit social process all contribute to keep the cost of changes low." By ensure compatabilty and security of design proces each element of technology required in highly scrutinized to follow XP practices.
- **Project Implementation Example**: Our team has chosen to use GLSL as our graphics language which will work on a majority all all modern GPU's. This gives us flexibilty to develop and test across a range of different machines and see how well the GPU's handle the changes.

### b. Identify candidate alternatives for the software system elements.

- **Outcome Addressed**: e
- **Extreme Programming Implementation**: "As a direction for improvement, incremental design doesn’t say that designing in advance of experience is horrible. It says that design done close to when it is used is more efficient." If the team has analyzed the best options and chosen the one that best enables their project to succed then they are following XP practices.
- **Project Implementation Example**: Our client has requested that we develop with rust and from there we explored the best library to use with it to integrate acess within the GPU. After exploring all uptions we chose glium as it provided a wide range of availible integration and ways to conform the code to our need.

### c. Assess each candidate alternative against criteria developed from expected design characteristics and element requirements to determine suitability for the intended application.

- **Outcome Addressed**: f
- **Extreme Programming Implementation**: "As your expertise grows in making changes to a running system in small, safe steps, you can afford to defer more and more of the design investment." XP focues on improving you understanding of the steps you are taking and then directly doing them.
- **Project Implementation Example**: Our cirteria for development was to have an easy to learn/get started library for people unfamiliar with rust as well as integration to freely change Vertex's as much as we want in our shaders. We all needed something that was free, easy to implement, and well documented. Out of the library's we explored glium addressed all of these concerns.

### d. Choose the preferred alternatives among candidate design solutions for the software system elements.

- **Outcome Addressed**: g
- **Extreme Programming Implementation**: "Once you see an idea for improvement that makes sense to you, do it.". XP focues on the idea if something looks good try it and see what happens. If it works congrats, if not you learned immedietly try something else. When a candidte solution is chosen see if it works and if not choose another.
- **Project Implementation Example**: We chose rust glium to develop with in our rust project as it fix the criteria we assigned. We have continued to develop in it and have not encounted any problems or limitations of the library that would prevent us from implementing our idea or reasons to switch to another project.

## 4. Manage the design.

### a. Capture the design and rationale.

- **Outcome Addressed**: f
- **Extreme Programming Implementation**: "You can’t make a good decision based on image alone. To choose a car wisely you need to know your constraints, both cost and intended use. All other things being equal, appeal comes into play." Using stories allows for team to quickly analyze the cost and what needs to be implemented. This quick apporach to understanding what needs to happen each cycle and being able to keep track of it is XP.
- **Project Implementation Example**: We create stories and tasks withing our github repo under milestones and issues. This lets us capture the current design of the system and the changes we need to implement.

### b. Establish traceability between the detailed design elements, the system/software requirements, and the architectural entities of the software system architecture.

- **Outcome Addressed**: h
- **Extreme Programming Implementation**: "Extensive internal documentation of software is an example of a practice that violates mutual benefit." In XP all documentation should be in the code or auto generated.
- **Project Implementation Example**: From our client meetings we get the requirements from our client and record them in stories on our repo. From here as we develop code we update the issues on on repo so we can trace back whre code was changed and why. Using the github repo we are able to go back and explore code changes/history to see how our proect developed.

### c. Determine the status of the software system and element design.

- **Outcome Addressed**: b
- **Extreme Programming Implementation**:"It’s easy to get carried away programming and put in code “just in case.” By stating explicitly and objectively what the program is supposed to do, you give yourself a focus for your coding. If you really want to put that other code in, write another test after you’ve made this one work." (Beck and Andres, 30)
- **Project Implementation Example**: In our project we verify our design and code through extensive testing. We follow XP's principle of test-first deisng where we first write tests so that we know what our system needs to implement. After this we develop code for our project to that will pass our tests. While following these guidlines we ensure that the status of our system is of high quality and meets standards.

### d. Provide key artifacts and information items that have been selected for baselines.

- **Outcome Addressed**: g
- **Extreme Programming Implementation**: In XP the baseline for everything is the code. It should be the documentation and example of how things should be implemented.
- **Project Implementation Example**: We don't devolop documentation other than our code base. Our code should be understandable/readable enough that any developer can look over it and understand what our project is doing.
