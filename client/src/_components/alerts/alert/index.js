import React, { Component } from 'react';
import { globalConstants } from '~/_constants';

class Alert extends Component {
	constructor(props) {
		super(props);
		this.state = {};
	}
	render() {
		return (
			<div className="container">
				<div className={"alert " + this.props.className} role="alert">
					{this.props.className === globalConstants.STYLES.ALERT_DANGER &&
						<strong>Oops! </strong>
					}
					{this.props.message}
				</div>
			</div>
		);
	}
}

export default Alert;
