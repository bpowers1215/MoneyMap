import React, { Component } from 'react';
import { connect } from 'react-redux';
import { globalConstants } from '~/_constants';
import AlertActions from '~/_actions/alert.actions';

const mapDispatchToProps = dispatch => {
	return {
		removeAlert: ({id}) => dispatch(AlertActions.removeAlert(parseInt(id, 10)))
	};
};

class ConnectedAlert extends Component {
	constructor(props) {
		super(props);
		this.state = {
			id: props.id
		};

		this.removeAlert = this.removeAlert.bind(this);
	}
	removeAlert() {
		this.props.removeAlert(this.state);
	}
	render() {
		return (
			<div className={"notification " + this.props.className}>
				<button className="delete" onClick={this.removeAlert}></button>
				{this.props.className === globalConstants.STYLES.ALERT_DANGER &&
					<strong>Oops! </strong>
				}
				{this.props.message}
			</div>
		);
	}
}

const Alert = connect(null, mapDispatchToProps)(ConnectedAlert);
export default Alert;