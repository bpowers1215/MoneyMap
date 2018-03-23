import React, { Component } from 'react';
import { connect } from 'react-redux';
import { EditableField, StaticField } from '~/_components/form';
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
		this.state = {};
	}
	componentWillMount(){
		this.props.dispatch(UserActions.getAccount());
	}
	render() {
		return (
			<React.Fragment>
				<h4 className="subtitle is-5">Your Account Information</h4>
				<EditableField
					type="input"
					name="firstName"
					fieldId="accountFirstName"
					label="First Name"
					placeholder="First Name"
					value={this.props.firstName}>
				</EditableField>
				<EditableField
					type="input"
					name="lastName"
					fieldId="accountLastName"
					label="Last Name"
					placeholder="Last Name"
					value={this.props.lastName}>
				</EditableField>
				<StaticField
					label="First Name"
					value={this.props.firstName}>
				</StaticField>
			</React.Fragment>
		);
	}
}

const AccountInfo = connect(mapStateToProps)(ConnectedAccountInfo);
export default AccountInfo;
