import React, { Component } from 'react';
import PropTypes from 'prop-types';


class EditableForm extends Component {
	constructor(props) {
		super(props);
		this.state = {
			editEnabled: props.editEnabled
		};
		this.submitFormAction = this.submitFormAction.bind(this);
	}
	componentWillReceiveProps(nextProps){
		let { editEnabled } = nextProps;
		let newState = {
			editEnabled: editEnabled
		}
		this.setState(newState);
	}
	submitFormAction(event){
		event.preventDefault();
		this.props.submitFormAction();
	}
	render() {
		return (
			<form onSubmit={this.submitFormAction}>
				{this.props.children}
				{ this.state.editEnabled &&
					<input type="submit" className="button is-primary" value="Submit"/>
				}
			</form>
		);
	}
}

EditableForm.propTypes = {
	submitFormAction: PropTypes.func.isRequired,
	editEnabled: PropTypes.bool.isRequired
}

export { EditableForm };
