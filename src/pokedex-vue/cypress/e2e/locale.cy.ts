describe('Locale Test', () => {
  it('visits the about page', () => {
    cy.visit('/about')
    cy.contains(':nth-child(2) > .mt-4', 'Browse your favourite pokemon')
  })

  it('change the locales', () => {
    cy.get('.navbar-end > :nth-child(2)').click()
    cy.contains(':nth-child(2) > .mt-4', 'Cari pokemon kesukaanmu')
  })
})
