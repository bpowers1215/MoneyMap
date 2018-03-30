import React, { Component } from 'react';
import PropTypes from 'prop-types';

/*
* Field Component
*/
class Field extends Component {
	constructor(props) {
		super(props);
		this.state = {}
	}
	render() {
		return (
			<div className="field">
				<label className="label">{this.props.label}</label>
				<div className={this.props.controlClasses}>
					<input 
						type={this.props.type}
						name={this.props.name}
						id={this.props.fieldId}
						placeholder={this.props.placeholder}
						onChange={this.props.onChange}
						required={this.props.required}
						value={this.props.value}
						className="input"
						autoFocus={this.props.autoFocus} />
					{this.props.children}
				</div>
			</div>
		)
	}
}

Field.defaultProps = {
	controlClasses: 'control',
	required: false,
	autoFocus: false
}

Field.propTypes = {
	type: PropTypes.string.isRequired,
	name: PropTypes.string.isRequired,
	fieldId: PropTypes.string.isRequired,
	label: PropTypes.string,
	value: PropTypes.string,
	controlClasses: PropTypes.string,
	placeholder: PropTypes.string.isRequired,
	onChange: PropTypes.func,
	required: PropTypes.bool,
	autoFocus: PropTypes.bool
}

export { Field };