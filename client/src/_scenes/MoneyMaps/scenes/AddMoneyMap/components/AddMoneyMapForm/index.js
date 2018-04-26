import React, { Component } from 'react';
import { connect } from 'react-redux';
import { EditableField, EditableForm } from '~/_components/form';
import MoneyMapActions from '~/_scenes/MoneyMaps/data/actions';

const mapDispatchToProps = dispatch => {
	return {
		createMoneyMap: (moneyMap) => dispatch(MoneyMapActions.createMoneyMap(moneyMap, '/money_maps'))
	};
};

const mapStateToProps = state => {
	return {}
};

class ConnectedAddMoneyMapForm extends Component {
	constructor(props) {
		super(props);
		this.state = {
			moneyMap: {
				name: ''
			}
		};
		this.handleChange = this.handleChange.bind(this);
		this.createMoneyMap = this.createMoneyMap.bind(this);
	}
	handleChange(event){
		let newState = Object.assign({}, this.state);
		newState.moneyMap[event.target.name] = event.target.value;
		this.setState(newState);
	}
	createMoneyMap(){
		this.props.createMoneyMap(this.state.moneyMap);
	}
	render() {
		return (
			<EditableForm submitFormAction={this.createMoneyMap} editEnabled={true}>
				<EditableField
					type="input"
					name="name"
					fieldId="moneyMapName"
					label="Name"
					placeholder="Name"
					editAllowed={true}
					alwaysEditable={true}
					onChange={this.handleChange}>
				</EditableField>
			</EditableForm>
		);
	}
}

const AddMoneyMapForm = connect(mapStateToProps, mapDispatchToProps)(ConnectedAddMoneyMapForm);

export default AddMoneyMapForm;
