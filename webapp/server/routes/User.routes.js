const router = require('express').Router();
let User = require('../models/User.model');

router.post('/', (req, res) => {
    const { username, firstname, lastname, email, password, company } = req.body;
    if (!username || !firstname || !lastname || !email || !password || !company) {
      return res.status(400);
    }
  }
);

router.route('/auth').get((req, res) => {
  User.findOne({
    username: req.username,
    password: req.password 
  })
    .then(user => res.json(user))
    .catch(err => res.status(400).json(`Error: ${err}`));
});

module.exports = router;