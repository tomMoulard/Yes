# Makefile for building and running the Bidding App

# Variables
FRONTEND_DIR := frontend
BACKEND_DIR := backend
TERRAFORM_DIR := terraform

# Targets
.PHONY: all frontend backend dev live

all: frontend backend

frontend:
	cd $(FRONTEND_DIR) && $(MAKE)

backend:
	cd $(BACKEND_DIR) && $(MAKE)

dev:
	cd $(FRONTEND_DIR) && $(MAKE) dev &
	cd $(BACKEND_DIR) && $(MAKE) dev

live:
	cd $(FRONTEND_DIR) && $(MAKE)
	cd $(BACKEND_DIR) && $(MAKE)
	cd $(TERRAFORM_DIR) && terraform init && terraform apply -auto-approve
