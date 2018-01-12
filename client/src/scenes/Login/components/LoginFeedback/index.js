import React, { Component } from 'react';

class LoginFeedback extends Component {
	constructor(props) {
		super(props);
		this.state = {};
	}
	render() {
		return (
			<div className="container">
				<div>{this.props.data.username}</div>
			</div>
		);
	}
}

export default LoginFeedback;
