import React, { Component } from 'react';
import { connect } from 'react-redux';
import UserActions from '~/_data/users/actions';

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
		console.log(props);
		this.state = {};
	}
	componentWillMount(){
		this.props.dispatch(UserActions.getAccount());
	}
	render() {
		return (
			<React.Fragment>
				<h4 className="subtitle is-5">Your Account Information</h4>
				<div className="field">
					<label className="label label-static">First Name</label>
					<div className="control">
						<p>{this.props.firstName}</p>
					</div>
				</div>
				<div className="field">
					<label className="label label-static">Last Name</label>
					<div className="control">
						<p>{this.props.lastName}</p>
					</div>
				</div>
				<div className="field">
					<label className="label label-static">Email</label>
					<div className="control">
						<p>{this.props.email}</p>
					</div>
				</div>
			</React.Fragment>
		);
	}
}

const AccountInfo = connect(mapStateToProps)(ConnectedAccountInfo);
export default AccountInfo;
