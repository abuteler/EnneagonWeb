const express = require('express');
const cors = require('cors');
const mongoose = require('mongoose');

require('dotenv').config();

const app = express();
const port = process.env.PORT || 9000;
app.use(cors());
app.use(express.json());

const uri = process.env.ATLAS_URI || 'mongodb://127.0.0.1:27017/timelogs';
mongoose.connect(uri, {
    useNewUrlParser: true,
    useCreateIndex: true,
    useUnifiedTopology: true
});

const connection = mongoose.connection;
connection.once('open', function() {
    console.log("MongoDB database connection established successfully");
});

app.use('/users', require('./routes/User.routes'));

app.listen(port, function() {
    console.log(`Server is running on Port: ${port}. Hello you.`);
});
