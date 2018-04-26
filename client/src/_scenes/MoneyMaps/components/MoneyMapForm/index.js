import React, { Component } from 'react';
import { connect } from 'react-redux';
import PropTypes from 'prop-types';
import { EditableField, EditableForm } from '~/_components/form';
import MoneyMapFormActions from './actions';

const mapDispatchToProps = dispatch => {
	return {
		enableEditableForm: () => dispatch(MoneyMapFormActions.enableEditableForm())
	};
};

const mapStateToProps = state => ({
	editEnabled: state.scenes.users.account.accountInfo.editEnabled
});

class ConnectedMoneyMapForm extends Component {
	constructor(props) {
		super(props);
		this.state = {
			moneyMap: {
				name: ""
			},
			editEnabled: props.editEnabled
		};
		this.handleChange = this.handleChange.bind(this);
		this.enableEdit = this.enableEdit.bind(this);
		this.updateMoneyMap = this.updateMoneyMap.bind(this);
	}
	componentWillMount(){
		this.props.getMoneyMap();
	}
	componentWillReceiveProps(nextProps){
		let { name, editEnabled } = nextProps;
		let newState = {
			moneyMap: {
				name
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
	updateMoneyMap(){
		this.props.updateMoneyMap(this.state.accountDetails);
	}
	render() {
		return (
			<EditableForm submitFormAction={this.updateMoneyMap} editEnabled={this.state.editEnabled}>
				<EditableField
					type="input"
					name="name"
					fieldId="moneyMapName"
					label="Name"
					placeholder="Name"
					editEnabled={this.state.editEnabled}
					onEdit={this.enableEdit}
					onChange={this.handleChange}
					value={this.state.moneyMap.name}>
				</EditableField>
			</EditableForm>
		);
	}
}

ConnectedMoneyMapForm.defaultProps = {
	getMoneyMap: () => {},
	editEnabled: false
}

ConnectedMoneyMapForm.propTypes = {
	getMoneyMap: PropTypes.func,
	updateMoneyMap: PropTypes.func.isRequired,
	editEnabled: PropTypes.bool
}

const MoneyMapForm = connect(mapStateToProps, mapDispatchToProps)(ConnectedMoneyMapForm);
export default MoneyMapForm;
