const mqtt = require('mqtt')
const client = mqtt.connect('mqtt://127.0.0.1:1883')

let measurements = {
  "temperature": 30,
  "humidity": 30
}

client.on('connect', function () {
  console.log("Connected")

  setInterval(function () {

    //randomly change the values of the measurements, single change can be max of 0.1
    measurements.temperature += Math.random() * 0.2 - 0.1;
    measurements.humidity += Math.random() * 0.2 - 0.1;

    //round the values to 2 decimal places
    measurements.temperature = Math.round(measurements.temperature * 100) / 100;
    measurements.humidity = Math.round(measurements.humidity * 100) / 100;

    //publish the measurements
    client.publish('/sensor1', JSON.stringify(measurements));
  }, 10_000)

  // client.subscribe('/#', function (err) {
  //   if (!err) {
  //     client.publish('presence', 'Hello mqtt')
  //   }
  // })
})

client.on('message', function (topic, message) {
  // message is Buffer
  console.log(topic.toString() + ": " + message.toString())
  //   client.end()
})