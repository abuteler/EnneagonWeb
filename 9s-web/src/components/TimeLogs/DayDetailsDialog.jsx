import React from 'react';

import Dialog from '@material-ui/core/Dialog';
import DialogTitle from '@material-ui/core/DialogTitle';
import DialogContent from '@material-ui/core/DialogContent';
import DialogActions from '@material-ui/core/DialogActions';
import Button from '@material-ui/core/Button';
import IconButton from '@material-ui/core/IconButton';
import { Close, Update } from '@material-ui/icons/';
import Typography from '@material-ui/core/Typography';
import { Grid } from '@material-ui/core';

import './DayDetailsDialog.scss';


class DayDetailsDialog extends React.Component {

  render () {
    const {
      title,
      timeEntries,
      totalMinutesWorked,
      handleClose,
      open,
    } = this.props;
    const minutos = totalMinutesWorked % 60;
    const horas = (totalMinutesWorked - minutos) / 60;

    return (
      <Dialog aria-labelledby="day-details-dialog-title" className="DayDetailsDialog" open={open} onClose={handleClose}>
        <DialogTitle id="day-details-dialog-title" variant="h6" onClose={handleClose}>
          {title}
          <IconButton aria-label="close" onClick={handleClose} className="closeIcon">
            <Close />
          </IconButton>
        </DialogTitle>
        <DialogContent dividers>
          <Grid container>
            <Grid item xs={3}>
              <Typography gutterBottom className="bold-text">Time</Typography>
            </Grid>
            <Grid item xs={9}>
              <Typography gutterBottom className="bold-text">Tasks</Typography>
            </Grid>
            { timeEntries.map((entry, i)=>(
              <Grid item container key={i}>
                <Grid item xs={3}>
                  <span className="TimeLapse">{entry.in} - {entry.out}</span>
                </Grid>
                <Grid item xs={9}>
                  <ul className="TaskList">
                    { entry.tasks.map((task, ix)=>(
                      <li key={ix}><Typography gutterBottom>{task}</Typography></li>
                    ))}
                  </ul>
                </Grid>
              </Grid>
            ))}
            <Grid item>
              <Typography gutterBottom className="totalTimeContainer">
                <span className="bold-text">Total:</span>&nbsp;{horas}h{minutos > 0 && ` ${minutos}'`}&nbsp;<Update className="clockIcon" />
              </Typography>
            </Grid>
          </Grid>
        </DialogContent>
        <DialogActions>
          <Button autoFocus onClick={handleClose} color="primary">
            Cerrar
          </Button>
        </DialogActions>
      </Dialog>
    );
  }
}

export default DayDetailsDialog;
