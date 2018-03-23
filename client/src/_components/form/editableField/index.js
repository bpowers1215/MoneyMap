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
			isEdit: true
		}

		this.editField = this.editField.bind(this);
	}
	editField(){
		this.setState({isEdit:true});
	}
	render() {
		if (this.state.isEdit) {
			return (
				<Field
					type={this.props.type}
					name={this.props.name}
					fieldId={this.props.fieldId}
					label={this.props.label}
					value={this.props.value}
					controlClasses={this.props.controlClasses}
					placeholder={this.props.placeholder}
					onChangeHandler={this.onChangeHandler}
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
	autoFocus: false
}

EditableField.propTypes = {
	type: PropTypes.string.isRequired,
	name: PropTypes.string.isRequired,
	fieldId: PropTypes.string.isRequired,
	label: PropTypes.string,
	value: PropTypes.string,
	controlClasses: PropTypes.string,
	placeholder: PropTypes.string.isRequired,
	onChangeHandler: PropTypes.func,
	required: PropTypes.bool,
	autoFocus: PropTypes.bool
}

export { EditableField };