# Project Description - IoT Gateway

The Vinesia IoT Gateway is a system designed to connect the Internet of Things (IoT) devices with the canisters on the Internet Computer. It enables both inbound and outbound traffic. The project is composed of three components: the gateway, candid specification, and example storage canister. The gateway facilitates the transfer of data between IC and MQTT and requires configuration to connect to a specific MQTT broker. The candid specification provides a standardized method for IC canisters to interact with the gateway. When set up correctly, the canister can both receive and send messages to the MQTT protocol broker on designated topics. The example storage canister offers a straightforward demonstration of how to receive, send, and store data using the gateway.

In this project, we will explore the integration of canisters with IoT devices for data storage and display. The adoption of canisters will not only enable secure and cost-effective storage but also ensure decentralization, true to the spirit of blockchain technology. The stored data will also be readily available for display and use without dependence on a centralized hosting provider.

Through this project, we aim to set a benchmark for storing and displaying IoT metrics data in a decentralized manner using a real-world use case. We believe this endeavor will advance the utilization of blockchain technology for data storage, marking a significant leap towards true decentralization. Ultimately, this project serves a broader vision of catalyzing the digital revolution by leveraging the immense potential of IoT data.

# TimeDB

TimeDB is a time-series database, designed for high-performance data handling and querying. This project is implemented in Rust. It is designed to store data in memory and enables to query it. 

Currently data can be added and queried via the IC. It can also be queried via http protocol.

## Query Method

Querying of TimeDB is done using a list of Actions

### Action

The `Action`` structure in TimeDB is pivotal for querying data and is defined with the following variants:

- Range(u64, Option<u64>): Represents a range query based on timestamps. It accepts a start timestamp and an optional end timestamp. If the end timestamp is not provided, it defaults to the maximum possible value, indicating an open-ended range.

- Filter(Expression): Enables filtering of results based on a specified Expression. This variant suggests the ability to apply complex filtering criteria to the data.

- AggregateWindow(String, AggregateFunction): Facilitates aggregation of data within a specified window. The first parameter likely represents the window specification (such as time intervals), while the second parameter is an AggregateFunction, indicating the type of aggregation to be performed (like sum, average, etc.).

### Expression

The Expression structure from the expression.rs file in the TimeDB project is designed to represent various types of expressions used for querying and filtering data. It is an enumeration (enum) with several variants, each tailored to a specific kind of expression or operation. Here's a detailed breakdown:

These expressions are essential for constructing complex queries and filters in the TimeDB system. They enable users to specify detailed conditions for data retrieval, such as selecting entries based on specific field values, combining multiple conditions, and excluding or including certain fields or tags.

For example, to filter entries where a field "temperature" is greater than 20, you might create an expression like Expression::Gt("temperature".to_string(), Value::Int(20)).

### Variants of Expression:

- Eq(String, Value): Represents an equality condition. The first parameter is a string representing the field or tag name, and the second parameter is the value to which the field or tag should be equal.

- Gt(String, Value): Stands for 'greater than'. It compares the specified field or tag (first parameter) to the provided value (second parameter) and checks if it is greater.

- Lt(String, Value): Represents 'less than'. Similar to Gt, but checks if the field or tag is less than the given value.

- Ge(String, Value): 'Greater than or equal to'. It compares the field or tag to the value, checking for greater than or equality.

- Le(String, Value): 'Less than or equal to'. Similar to Ge, but for less than or equal conditions.

- And(Expression, Expression): Logical AND operation. Combines two expressions and evaluates to true if both expressions are true.

- Or(Expression, Expression): Logical OR operation. Evaluates to true if either of the two expressions is true.

- Not(Expression): Logical NOT operation. Inverts the result of the expression it contains.

- TagFilter(Vec<String>): Used to filter entries based on tags. The vector contains strings representing the tags to keep.

- FieldFilter(Vec<String>): Similar to TagFilter, but for fields. The vector contains strings representing the fields to keep.


## Target Canister Specifics

- update: insert(measurement: string, entry: Entry) - Inserts single Entry to TimeDB
- update: insert_bulk(measurement: string, entries: Entry[]) - Inserts Multiple Entires to TimeDB

- query: run_query(measurement: string, actions: Action[]) - Runs query composed of several actions against data in measurement
- query: get_settings(): Settings - returns canisters settings related to MQTT channels processing

---

# Requirements
- DFX - 0.14.3
- RUST - v1.69 or higher with wasm32-unknown-unknown target