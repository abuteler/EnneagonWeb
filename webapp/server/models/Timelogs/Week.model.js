const mongoose = require('mongoose');
const Schema = mongoose.Schema;
const Day = require('Day.model');

let WeekSchema = new Schema({
  days: [Day],
  maxlength: 7,
}, {
  timestamps: true,
});

const Week = mongoose.model('Week', WeekSchema);

module.exports = Week;