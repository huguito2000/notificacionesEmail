import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

response = WS.sendRequest(findTestObject('Generales/PassWordReset/passwordRestCand'))

statusCode = WS.getResponseStatusCode(response)

println(statusCode)

WS.verifyResponseStatusCode(response, 200)

WebUI.openBrowser('https://yopmail.com/es/')

WebUI.setText(findTestObject('Generales/Cuenta Bloqueada/Campo Email'), 'huguito.candidato')

WebUI.sendKeys(findTestObject('Generales/Cuenta Bloqueada/Campo Email'), Keys.chord(Keys.ENTER))

WebUI.takeScreenshot('/Users/huguito/Desktop/notificaciones/Generales/paswordResetCand.png')

WebUI.delay(5)

WebUI.closeBrowser()

