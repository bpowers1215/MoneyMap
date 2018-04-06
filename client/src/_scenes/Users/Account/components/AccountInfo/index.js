import React, { Component } from 'react';
import { connect } from 'react-redux';
import { EditableField, StaticField } from '~/_components/form';
import UserActions from '~/_data/users/actions';
import AccountInfoActions from './actions';

const mapDispatchToProps = dispatch => {
	return {
		enableEditableForm: () => dispatch(AccountInfoActions.enableEditableForm()),
		getAccount: () => dispatch(UserActions.getAccount()),
		updateAccount: (accountDetails) => dispatch(UserActions.updateAccount(accountDetails))
	};
};

const mapStateToProps = state => ({
	email: state.data.users.auth.email,
	id: state.data.users.auth.id,
	firstName: state.data.users.auth.firstName,
	lastName: state.data.users.auth.lastName,
	token: state.data.users.auth.token,
	editEnabled: state.scenes.users.account.accountInfo.editEnabled
});

class ConnectedAccountInfo extends Component {
	constructor(props) {
		super(props);
		this.state = {
			accountDetails: {
				firstName: "",
				lastName: ""
			},
			editEnabled: props.editEnabled
		};
		this.handleChange = this.handleChange.bind(this);
		this.updateAccount = this.updateAccount.bind(this);
		this.enableEdit = this.enableEdit.bind(this);
	}
	componentWillMount(){
		this.props.getAccount();
	}
	componentWillReceiveProps(nextProps){
		let { firstName, lastName, editEnabled } = nextProps;
		let newState = {
			accountDetails: {
				firstName,
				lastName
			},
			editEnabled: editEnabled
		}
		this.setState(newState);
	}
	enableEdit(){
		this.props.enableEditableForm();
	}
	handleChange(event){
		let newState = Object.assign({}, this.state);
		newState.accountDetails[event.target.name] = event.target.value;
		this.setState(newState);
	}
	updateAccount(){
		this.props.updateAccount(this.state.accountDetails);
	}
	render() {
		return (
			<React.Fragment>
				<EditableField
					type="input"
					name="firstName"
					fieldId="accountFirstName"
					label="First Name"
					placeholder="First Name"
					editEnabled={this.state.editEnabled}
					onEdit={this.enableEdit}
					onChange={this.handleChange}
					value={this.state.accountDetails.firstName}>
				</EditableField>
				<EditableField
					type="input"
					name="lastName"
					fieldId="accountLastName"
					label="Last Name"
					placeholder="Last Name"
					editEnabled={this.state.editEnabled}
					onEdit={this.enableEdit}
					onChange={this.handleChange}
					value={this.state.accountDetails.lastName}>
				</EditableField>
				<StaticField
					label="Email"
					value={this.props.email}>
				</StaticField>
				{ this.state.editEnabled &&
					<a className="button is-primary" onClick={this.updateAccount}>Update</a>
				}
			</React.Fragment>
		);
	}
}

const AccountInfo = connect(mapStateToProps, mapDispatchToProps)(ConnectedAccountInfo);
export default AccountInfo;
