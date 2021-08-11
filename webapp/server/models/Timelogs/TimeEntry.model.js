const mongoose = require('mongoose');
const Schema = mongoose.Schema;

let TimeEntrySchema = new Schema({
  in: { type: String, required: false },
  out: { type: Date, required: true },
  tasks: [String]
}, {
  timestamps: true,
});

const TimeEntry = mongoose.model('TimeEntry', TimeEntrySchema);

module.exports = TimeEntry;
