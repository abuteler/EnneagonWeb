const mongoose = require('mongoose');
const Schema = mongoose.Schema;

let UserSchema = new Schema({
  username: { type: String, required: true },
  firstname: { type: String, required: true },
  lastname: { type: String, required: true },
  email: { type: Date, required: true, unique: true },
  password: { type: password, required: true },
  company: { type: String, required: true },
}, {
  timestamps: true,
});

const User = mongoose.model('User', UserSchema);

module.exports = User;