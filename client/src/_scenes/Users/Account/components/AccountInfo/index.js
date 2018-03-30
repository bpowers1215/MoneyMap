import React, { Component } from 'react';
import { connect } from 'react-redux';
import { EditableField, StaticField } from '~/_components/form';
import UserActions from '~/_data/users/actions';

const mapDispatchToProps = dispatch => {
	return {
		getAccount: () => dispatch(UserActions.getAccount()),
		updateAccount: (accountDetails) => dispatch(UserActions.updateAccount(accountDetails))
	};
};

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
		this.state = {
			firstName: "",
			lastName: ""
		};
		this.handleChange = this.handleChange.bind(this);
		this.updateAccount = this.updateAccount.bind(this);
	}
	componentWillMount(){
		this.props.getAccount();
	}
	componentWillReceiveProps(nextProps){
		let { firstName, lastName } = nextProps;
		this.setState({ firstName, lastName });

	}
	handleChange(event){
		let newState = {};
		newState[event.target.name] = event.target.value;
		this.setState(newState);
	}
	updateAccount(){
		this.props.updateAccount(this.state);
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
					onChange={this.handleChange}
					value={this.state.firstName}>
				</EditableField>
				<EditableField
					type="input"
					name="lastName"
					fieldId="accountLastName"
					label="Last Name"
					placeholder="Last Name"
					onChange={this.handleChange}
					value={this.state.lastName}>
				</EditableField>
				<StaticField
					label="Email"
					value={this.props.email}>
				</StaticField>
				<a className="button is-primary" onClick={this.updateAccount}>Update</a>
			</React.Fragment>
		);
	}
}

const AccountInfo = connect(mapStateToProps, mapDispatchToProps)(ConnectedAccountInfo);
export default AccountInfo;
