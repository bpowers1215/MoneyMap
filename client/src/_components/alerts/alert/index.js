import React, { Component } from 'react';

class Alert extends Component {
	constructor(props) {
		super(props);
		this.state = {};
	}
	render() {
		return (
			<div className="container">
				<div className="alert alert-danger" role="alert">
					<strong>Oh snap!</strong> {this.props.message}
				</div>
			</div>
		);
	}
}

export default Alert;
