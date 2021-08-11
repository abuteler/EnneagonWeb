const mongoose = require('mongoose');
const Schema = mongoose.Schema;
const TimeEntry = require('TimeEntry.model');

let DaySchema = new Schema({
  name: { type: String, required: false },
  date: { type: Date, required: true },
  worklog: [TimeEntry]
}, {
  timestamps: true,
});

const Day = mongoose.model('Day', DaySchema);

module.exports = Day;