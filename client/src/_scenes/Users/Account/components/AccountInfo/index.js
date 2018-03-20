import React, { Component } from 'react';
import { connect } from 'react-redux';

const mapStateToProps = state => ({
		email: state.data.users.auth.email,
		id: state.data.users.auth.id,
		firstName: state.data.users.auth.firstName,
		lastName: state.data.users.auth.lastName,
		token: state.data.users.auth.token
});

class ConnectedAccountInfo extends Component {
	constructor(props) {
		super(props);
		this.state = {};
	}
	componentWillMount(){

	}
	render() {
		return (
			<React.Fragment>
				<div>id: {this.props.id}</div>
				<div>email: {this.props.email}</div>
				<div>first name: {this.props.firstName}</div>
				<div>last name: {this.props.lastName}</div>
				<div>token: {this.props.token}</div>
			</React.Fragment>
		);
	}
}

const AccountInfo = connect(mapStateToProps)(ConnectedAccountInfo);
export default AccountInfo;
