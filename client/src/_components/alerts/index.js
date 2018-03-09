import React, { Component } from 'react';
import { connect } from 'react-redux';
import Alert from './alert';
import './styles.scss';

const mapStateToProps = state => {
	return ({
		alerts: ( state.alerts.alerts ? state.alerts.alerts : [] )
})
};

class ConnectedAlerts extends Component {
	constructor(props) {
		super(props);
		this.state = {};
	}
	render() {
		if ( this.props.alerts.length > 0 ) {
			let alerts = [];
			for ( let alert in this.props.alerts) {
				alerts.push (
					<Alert key={alert} id={alert} className={this.props.alerts[alert].className} message={this.props.alerts[alert].message} />
				);
			}
			return (
				<div className="container" id="notifications">
					{alerts}
				</div>
			);
		} else {
			return null;
		}
	}
}

const Alerts = connect(mapStateToProps)(ConnectedAlerts);
export default Alerts;
