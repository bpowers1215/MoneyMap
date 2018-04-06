import React, { Component } from 'react';
import PropTypes from 'prop-types';
import { Field, StaticField } from '../';
import './styles.scss';

/*
* EditableField Component
* Initially static label/value display. Selecting edit transfors value to an input
* field allowing changes to be made.
*/
class EditableField extends Component {
	constructor(props) {
		super(props);
		this.state = {
			isEdit: false
		}

		this.editField = this.editField.bind(this);
	}
	editField(){
		this.props.onEdit();
		this.setState({isEdit:true});
	}
	componentWillReceiveProps(nextProps){
		let { editEnabled } = nextProps;
		
		// If edit is disabled, reset isEdit state
		if (!editEnabled) {
			let newState = {
				isEdit: false
			}
			this.setState(newState);
		}
	}
	render() {
		if (this.state.isEdit && this.props.editEnabled) {
			return (
				<Field
					type={this.props.type}
					name={this.props.name}
					fieldId={this.props.fieldId}
					label={this.props.label}
					value={this.props.value}
					controlClasses={this.props.controlClasses}
					placeholder={this.props.placeholder}
					onChange={this.props.onChange}
					autoFocus>
				</Field>
			)
		} else {
			return (
				<StaticField
					label={this.props.label}
					value={this.props.value}
					fieldClasses="field static-field editable"
					controlClasses={this.props.controlClasses}
					onClickHandler={this.editField}>
					<span className="icon is-small">
						<i className="fas fa-pencil-alt"></i>
					</span>
				</StaticField>
			)
		}
	}
}

EditableField.defaultProps = {
	controlClasses: 'control',
	required: false,
	autoFocus: false,
	editEnabled: false,
	onEdit: () => {}
}

EditableField.propTypes = {
	type: PropTypes.string.isRequired,
	name: PropTypes.string.isRequired,
	fieldId: PropTypes.string.isRequired,
	label: PropTypes.string,
	value: PropTypes.string,
	controlClasses: PropTypes.string,
	placeholder: PropTypes.string.isRequired,
	onChange: PropTypes.func,
	onEdit: PropTypes.func,
	required: PropTypes.bool,
	autoFocus: PropTypes.bool,
	editEnabled: PropTypes.bool
}

export { EditableField };