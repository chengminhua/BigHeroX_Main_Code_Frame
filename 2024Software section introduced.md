# Software Section


----------


## ECS（Entity Component System）Design Structure
In our software, we use the Rust programming language and [the Bevy game engine](https://bevyengine.org), adopting an ECS (Entity Component System) design.

The ECS design has been applied in several games in recent years and has performed well.

In the ECS design, all content in the code is split into several Entities and Systems:

	•	Each Entity carries its own Components, which are used to store data.
	•	Systems can query Entities that carry specific Components and can read and write the Components of specified Entities, add or remove Components from specified Entities, and create or destroy Entities.
	•	The ECS engine periodically calls each System according to certain rules.


The benefits of this design are:

	•	Simple pattern, clear structure.
	•	High reuse through combination. Using composition instead of inheritance, any Component can be assembled into any Entity like building blocks.
	•	Strong extensibility. Components and Systems can be added or removed at will. Because Components cannot directly access each other, and Systems cannot directly access each other, there is no coupling between Components or between Systems. There is also no design principle coupling between Systems and Components. For Systems, Components are just data placed aside; if the data provided by the Components is sufficient, the System updates, otherwise, it does not. So adding or removing any Component and System at any time will not cause program crashes or errors.
	•	Naturally compatible with DOP (data-oriented processing). Data is uniformly stored in various Components, and Systems directly process this data. The function call stack depth is greatly reduced, and the flow is weakened.
	•	Easy to optimize performance. Since data is uniformly stored in Components, if all Components can be aggregated into contiguous memory in a reasonable manner, the CPU cache hit rate can be greatly improved. With good CPU cache hits, the traversal speed of Entities can be increased by 50 times, and the more objects there are, the more significant the performance improvement.
	•	Easy to implement multi-threading. Since Systems cannot directly access each other and are completely decoupled, theoretically, each System can be assigned a thread to run. It should be noted that the execution order of some Systems needs to be strictly specified, so the execution order should be considered when assigning threads to these Systems.

Of course, the ECS structure has some minor drawbacks：

	•	Writing code under these constraints can be slower at times. However, after getting used to it, the development speed will increase over time.
	•	If time-consuming operations are performed directly in the System, it may block the entire process. The solution is to move time-consuming operations to other threads or execute them asynchronously. Therefore, in actual development, the time consumption of each operation in the System needs to be well understood.




## Non-blocking Software and Hardware Communication

On the hardware side, we use the serial protocol for communication between the Windows host and the lower machine.

We use the [mio](https://crates.io/crates/mio) crate to handle communication between the Windows host and the lower machine.

For example, for read operations, we use a System that executes periodically at fixed intervals.

Each time this System executes, it checks whether the lower machine has returned data. If so, it extracts the data; if not, it skips and waits for the next execution of this System. This avoids using the traditional read function call method to prevent IO blocking, which could affect the execution of other Systems.



## System Execution Stages (Schedule)

In Bevy, the execution order of Systems is divided into stages, and each System should be assigned to a stage when added.


The common stages provided by Bevy are:

- `Startup`: Executes only once when the program starts.

- `PreUpdate`,`Update`,and`PostUpdate`：: Execute one round in the order mentioned above while rendering a frame, so the number of executions is the same as the number of rendered frames. `PreUpdate` executes before each frame is rendered, and `PostUpdate` executes after rendering.

- `FixedPreUpdate`,`FixedUpdate`,and`FixedPostUpdate`：Execute one round in the order mentioned above at a fixed frequency. The default frequency of Bevy is 64 times per second. In the current version of the program, we use a frequency of 500 times per second.




## System  Overview

Here are some key Systems.

The following System names may not exactly match the function names in the actual code and are for reference only.




### `FixedPreUpdate` Stage：

- `net_fetch_from_coach_system`: Reads data and instructions returned from the coach machine.

- `read_panorama_camera_system`: Reads image data transmitted from the panoramic camera on top of the robot.

- `load_panorama_data_system`:Parses the image data transmitted from the panoramic camera to obtain information such as the current position, ball position, and opponent positions on the field.

- `read_com_robot_system`: Reads data transmitted from the robot’s lower machine.

- `read_com_mpu_system`: Reads data transmitted from the robot’s MPU module.

If new information acquisition devices, such as front-facing cameras, are added in the future, corresponding Systems for reading these devices can be added in this stage.




### `FixedUpdate` Stage：

- `set_motion_target_system`：: Processes the information obtained in the current round of the `FixedPreUpdate` stage to determine the target point for the robot to move to.

- `slam_motion_plan_system`：Based on the robot’s current position, target position, and field information, performs path planning to obtain the actual movement instructions the robot should execute.

If new algorithms are introduced to assist decision-making in the future, new Systems can be added in this stage.

### `FixedPostUpdate` Stage：

- `robot_motion_system`:Transmits motion instructions to the robot’s engines, i.e., the speeds of the motion wheels and ball-suction wheels.

- `robot_kick_system`: Transmits shooting instructions to the robot’s shooting rod.

If new controllable devices, such as telescopic rods, are added in the future, corresponding Systems for controlling these devices can be added in this stage.