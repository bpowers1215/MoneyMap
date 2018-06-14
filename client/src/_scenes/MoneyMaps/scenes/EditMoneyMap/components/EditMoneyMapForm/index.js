import React, { Component } from 'react';
import { connect } from 'react-redux';
import PropTypes from 'prop-types';
import { EditableField, EditableForm } from '~/_components/form';
import MoneyMapActions from '~/_scenes/MoneyMaps/data/actions';
import EditMoneyMapFormActions from './actions';
import { history } from '~/_helpers/history';

const mapDispatchToProps = dispatch => {
	return {
		enableEditableForm: () => dispatch(EditMoneyMapFormActions.enableEditableForm()),
		getMoneyMap: () => dispatch(MoneyMapActions.getMoneyMaps()),
		updateMoneyMap: (moneyMap) => dispatch(MoneyMapActions.updateMoneyMap(moneyMap, '/money_maps')),
		cantFindMoneyMap: () => {
			dispatch(MoneyMapActions.cantFindMoneyMap('/money_maps'));
			history.push('/money_maps');
		},
	};
};

const mapStateToProps = state => {
	return {
		moneyMaps: state.scenes.moneyMaps.data.moneyMaps,
		editEnabled: state.scenes.moneyMaps.editMoneyMap.moneyMapInfo.editEnabled
	}
};

class ConnectedEditMoneyMapForm extends Component {
	constructor(props) {
		super(props);
		this.state = {
			moneyMap: {
				id: '',
				name: ''
			},
			editEnabled: props.editEnabled
		};
		this.handleChange = this.handleChange.bind(this);
		this.updateMoneyMap = this.updateMoneyMap.bind(this);
		this.enableEdit = this.enableEdit.bind(this);
	}
	componentWillMount(){
		this.props.getMoneyMap();
	}
	componentWillReceiveProps(nextProps){
		let { moneyMapId, moneyMaps, editEnabled } = nextProps;

		let newState = {
			moneyMap: moneyMaps[moneyMapId],
			editEnabled: editEnabled
		}
		this.setState(newState);
	}
	shouldComponentUpdate(nextProps, nextState) {
		let { moneyMapId, moneyMaps, editEnabled } = nextProps;
		
		if ( !(moneyMapId in moneyMaps) ) {
			this.props.cantFindMoneyMap();
			return false;
		}

		return true;
	}
	enableEdit(){
		this.props.enableEditableForm();
	}
	handleChange(event){
		let newState = Object.assign({}, this.state);
		newState.moneyMap[event.target.name] = event.target.value;
		this.setState(newState);
	}
	updateMoneyMap(){
		this.props.updateMoneyMap(this.state.moneyMap);
	}
	render() {
		if ( this.state.moneyMap )
			return (
				<EditableForm submitFormAction={this.updateMoneyMap} editEnabled={this.props.editEnabled}>
					<EditableField
						type="input"
						name="name"
						fieldId="moneyMapName"
						label="Name"
						placeholder="Name"
						editAllowed={this.state.editEnabled}
						onEdit={this.enableEdit}
						onChange={this.handleChange}
						value={this.state.moneyMap.name}>
					</EditableField>
				</EditableForm>
			);
		return null
	}
}

const EditMoneyMapForm = connect(mapStateToProps, mapDispatchToProps)(ConnectedEditMoneyMapForm);

EditMoneyMapForm.defaultProps = {
}

EditMoneyMapForm.propTypes = {
	moneyMapId: PropTypes.string.isRequired
}

export default EditMoneyMapForm;
